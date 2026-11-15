use axum::{Json, extract::State};
use cosmic_text::{
  Attrs,
  Buffer,
  CacheKeyFlags,
  Color,
  Family,
  FontSystem,
  Metrics,
  Shaping,
  SwashCache,
  fontdb::Database,
};
use image::{DynamicImage, Rgba, RgbaImage};
use uuid::Uuid;

use crate::{
  image::Image,
  state::{AppState, ImageWithId},
};

#[derive(Debug, serde::Deserialize)]
pub struct NewText {
  text: String,
}

#[axum::debug_handler]
pub async fn new_text(
  State(state): State<AppState>,
  Json(new_text): Json<NewText>,
) -> Json<ImageWithId> {
  let id = Uuid::now_v7();

  let original_file = CosmicTextRenderer::new().render(&new_text.text);

  let width = original_file.width();
  let height = original_file.height();

  let printer_ty = state.printer.lock().await.ty();
  let status = state.printer.lock().await.status();

  let length_mm = Image::calculate_length_mm(width, printer_ty);

  let processed_file = Image::process(original_file.clone(), status, 127, false);

  let image = Image {
    file_name: String::new(),
    width,
    height,
    original_width: width,
    original_height: height,
    length_mm,
    threshold: 127,
    inverted: false,
    original_file,
    processed_file,
  };

  state.add_image_with_id(image.clone(), id);

  Json(ImageWithId { id, image })
}

pub struct CosmicTextRenderer {
  swash_cache: SwashCache,
  font_system: FontSystem,
}

impl CosmicTextRenderer {
  pub fn new() -> Self {
    let swash_cache = SwashCache::new();
    let font_system = FontSystem::new_with_locale_and_db("en-GB".into(), {
      let mut db = Database::new();

      let fonts = [
        include_bytes!("../resources/NotJamOldStyle11/NotJamOldStyle14.ttf").to_vec(),
        include_bytes!("../resources/Undead Pixel Light 8/Undead Pixel 8 (with dot).ttf").to_vec(),
      ];

      for font in fonts {
        db.load_font_data(font);
      }

      db
    });

    Self {
      swash_cache,
      font_system,
    }
  }

  pub fn render(&mut self, text: &str) -> DynamicImage {
    let mut buffer = Buffer::new(&mut self.font_system, Metrics::new(14.0, 14.0));
    let mut buffer = buffer.borrow_with(&mut self.font_system);

    buffer.set_rich_text(
      [(text, Self::default_attrs())],
      &Attrs::new(),
      Shaping::Advanced,
      None,
    );

    buffer.shape_until_scroll(true);

    buffer.layout_runs();

    let (width, height) = buffer_dimensions(&buffer);

    let mut text = RgbaImage::new(width + 1, height);

    buffer.draw(
      &mut self.swash_cache,
      Color::rgb(0, 0, 0),
      |x, y, w, h, color| {
        let x = (x + 1) as u32;
        let y = y as u32;

        for w in 0..w {
          for h in 0..h {
            if x + w > width {
              continue;
            }

            text.put_pixel(
              x + w,
              y + h,
              Rgba([
                color.r(),
                color.g(),
                color.b(),
                if color.a() > 127 { 255 } else { 0 },
              ]),
            );
          }
        }
      },
    );

    DynamicImage::from(text)
  }

  fn default_attrs() -> Attrs<'static> {
    Attrs::new()
      .cache_key_flags(CacheKeyFlags::PIXEL_FONT)
      .metrics(Metrics::new(14.0, 14.0))
      .family(Family::Name("NotJamOldStyle14"))
  }
}

fn buffer_dimensions(buffer: &Buffer) -> (u32, u32) {
  let (width, height) = buffer
    .layout_runs()
    .map(|run| (run.line_w, run.line_height))
    .reduce(|(w1, h1), (w2, h2)| (w1.max(w2), h1 + h2))
    .unwrap_or((0.0, 0.0));

  ((width + 0.5).ceil() as u32, (height + 0.5).ceil() as u32)
}
