use std::path::PathBuf;

use axum::extract::Multipart;
use image::{
  DynamicImage,
  RgbaImage,
  imageops::{self, FilterType},
};
use ptouch_rs::{PrinterType, Status, TapeColor, TextColor};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Image {
  pub file_name: String,
  pub width: u32,
  pub height: u32,
  pub original_width: u32,
  pub original_height: u32,
  pub length_mm: u32,
  pub threshold: u8,
  pub inverted: bool,
  #[serde(skip)]
  pub original_file: DynamicImage,
  #[serde(skip)]
  pub processed_file: DynamicImage,
}

impl Image {
  pub fn set_threshold(&mut self, threshold: u8, status: Status) {
    self.threshold = threshold;

    self.processed_file = Image::process(
      self.original_file.clone(),
      status,
      self.threshold,
      self.inverted,
    );
  }

  pub fn set_inverted(&mut self, inverted: bool, status: Status) {
    println!("Setting inverted from {} to {}", self.inverted, inverted);
    self.inverted = inverted;

    self.processed_file = Image::process(
      self.original_file.clone(),
      status,
      self.threshold,
      self.inverted,
    );
  }

  fn process(
    image: impl Into<DynamicImage>,
    status: Status,
    threshold: u8,
    invert: bool,
  ) -> DynamicImage {
    let image = image.into();

    let colors = color_map(status.tape_color, status.text_color);
    let image = image.grayscale().into_rgba8();

    let width = image.width();
    let height = image.height();

    let tape_width = status.media_width.info().px;
    let ratio = (tape_width as f32) / (height as f32);

    let mut image = imageops::resize(
      &image,
      (width as f32 * ratio) as u32,
      tape_width,
      FilterType::Nearest,
    );

    image.pixels_mut().for_each(|pixel| {
      let is_text = if pixel.0[3] < 255 {
        false
      } else {
        pixel.0[0] < threshold
      };

      if is_text {
        pixel.0 = [colors.text.red, colors.text.green, colors.text.blue, 255];
      } else {
        pixel.0 = [colors.tape.red, colors.tape.green, colors.tape.blue, 255];
      }
    });

    if invert {
      imageops::invert(&mut image);
    }

    DynamicImage::from(image)
  }

  pub async fn from_multipart(
    printer_ty: PrinterType,
    status: Status,
    mut multipart: Multipart,
  ) -> Result<Self, ()> {
    let mut image = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
      match field.name() {
        Some("file") => {
          let content_type = field.content_type().unwrap();
          if !content_type.starts_with("image") {
            panic!("Not an image?");
          }

          let file_name = field
            .file_name()
            .map(|file_name| PathBuf::from(file_name).with_extension(""))
            .map(|file_name| file_name.to_str().unwrap().to_owned())
            .unwrap();

          let bytes = field.bytes().await.unwrap();
          let file = image::load_from_memory(&bytes).unwrap();

          let width = file.width();
          let height = file.height();

          image = Some((file_name, file, width, height));
        },
        _ => panic!(),
      }
    }

    let (file_name, original_file, original_width, original_height) = image.unwrap();

    let width = original_file.width();
    let height = original_file.height();

    let length_inch = (width as f32) / (printer_ty.info().dpi as f32);
    let length_mm = length_inch * 25.4;
    let length_mm = length_mm as u32;

    let processed_file = Image::process(original_file.clone(), status, 127, false);

    Ok(Image {
      file_name,
      width,
      height,
      original_width,
      original_height,
      length_mm,
      inverted: false,
      threshold: 127,
      original_file,
      processed_file,
    })
  }

  pub fn join<'a>(
    images: impl ExactSizeIterator<Item = &'a DynamicImage> + Clone,
    status: Status,
  ) -> RgbaImage {
    let colors = color_map(status.tape_color, status.text_color);

    let margin = 3;
    let total_margin = margin * 2 + 1;

    let image_num = images.len();

    let width: u32 = images
      .clone()
      .enumerate()
      .map(|(i, image)| {
        if i == 0 {
          image.width()
        } else {
          image.width() + total_margin
        }
      })
      .sum();

    let height = images
      .clone()
      .map(|image| image.height())
      .max()
      .unwrap_or_default();

    let mut joined = RgbaImage::new(width, height);
    for pixel in joined.pixels_mut() {
      pixel.0 = [colors.tape.red, colors.tape.green, colors.tape.blue, 255];
    }

    let mut separator = RgbaImage::new(1, height);
    for (i, pixel) in separator.pixels_mut().enumerate() {
      let color = if (i / 4).is_multiple_of(2) {
        colors.text
      } else {
        colors.tape
      };

      pixel.0 = [color.red, color.green, color.blue, 255];
    }

    let mut current_offset = 0;
    for (i, image) in images.enumerate() {
      imageops::overlay(&mut joined, image, current_offset as i64, 0);
      current_offset += image.width();

      if i != image_num - 1 {
        current_offset += margin;
        imageops::overlay(&mut joined, &separator, current_offset as i64, 0);
        current_offset += 1;
        current_offset += margin;
      }
    }

    joined
  }

  pub fn render<'a>(
    images: impl ExactSizeIterator<Item = &'a DynamicImage> + Clone,
    status: Status,
  ) -> RgbaImage {
    let colors = color_map(status.tape_color, status.text_color);
    let mut image = Image::join(images, status);

    image.pixels_mut().for_each(|pixel| {
      if pixel.0[0..3] == [colors.tape.red, colors.tape.green, colors.tape.blue] {
        pixel.0 = [255; 4];
      } else {
        pixel.0 = [0, 0, 0, 255];
      }
    });

    image
  }
}

struct ColorMap {
  tape: palette::Srgb<u8>,
  text: palette::Srgb<u8>,
}

fn color_map(tape: TapeColor, text: TextColor) -> ColorMap {
  let tape = match tape {
    TapeColor::None => palette::named::WHITE,
    TapeColor::BerryPink_TZe_MQP35 => todo!(),
    TapeColor::Black => palette::named::BLACK,
    TapeColor::Blue_TZe_5_345_5 => todo!(),
    TapeColor::Blue => palette::named::BLUE,
    TapeColor::Cleaning => palette::named::WHITE,
    TapeColor::Clear => palette::named::WHITE,
    TapeColor::ClearMatte => palette::named::WHITE,
    TapeColor::GoldSatin => palette::named::GOLD,
    TapeColor::Green => palette::named::GREEN,
    TapeColor::HeatShrinkTube => todo!(),
    TapeColor::Incompatible => todo!(),
    TapeColor::LightGray_TZe_MQL35 => todo!(),
    TapeColor::LimeGreen_TZe_MQG35 => todo!(),
    TapeColor::OrangeFluorescent => palette::named::ORANGE,
    TapeColor::Pink => palette::named::PINK,
    TapeColor::Red_TZe_435 => todo!(),
    TapeColor::Red => palette::named::RED,
    TapeColor::SilverMatte => palette::named::SILVER,
    TapeColor::SilverSatin => palette::named::SILVER,
    TapeColor::Stencil => palette::named::WHITE,
    TapeColor::White => palette::named::WHITE,
    TapeColor::WhiteFlexId => palette::named::WHITE,
    TapeColor::WhiteMatte => palette::named::WHITE,
    TapeColor::Yellow => palette::named::YELLOW,
    TapeColor::YellowFlexId => palette::named::YELLOW,
    TapeColor::YellowFluorescent => palette::named::YELLOW,
    TapeColor::Other => todo!(),
    TapeColor::Unknown(_) => todo!(),
  };

  let text = match text {
    TextColor::None => palette::named::BLACK,
    TextColor::Black => palette::named::BLACK,
    TextColor::Blue => palette::named::BLUE,
    TextColor::BlueF => todo!(),
    TextColor::Cleaning => palette::named::WHITE,
    TextColor::Gold => palette::named::GOLD,
    TextColor::Incompatible => todo!(),
    TextColor::Red => palette::named::RED,
    TextColor::Stencil => palette::named::BLACK,
    TextColor::White => palette::named::WHITE,
    TextColor::Other => todo!(),
    TextColor::Unknown(_) => todo!(),
  };

  ColorMap { tape, text }
}
