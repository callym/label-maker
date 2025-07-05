use std::io::Cursor;

use axum::{
  Json,
  Router,
  extract::{Multipart, Path, State},
  http::{StatusCode, header},
  response::IntoResponse,
  routing::{delete, get, post},
};
use ptouch_rs::{MediaType, PrinterType, TapeColor, TapeSize, TextColor};
use tower::ServiceBuilder;
use tower_http::{
  cors::CorsLayer,
  services::{ServeDir, ServeFile},
};

mod image;
mod state;

use ::image::ImageFormat;
use image::Image;
use state::AppState;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};
use uuid::Uuid;

use crate::state::ImageWithId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenvy::dotenv()?;

  let filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env()?;

  tracing_subscriber::fmt()
    .with_env_filter(filter)
    .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
    .try_init()
    .unwrap();

  let state = AppState::new().await?;

  let serve_dir =
    ServeDir::new("frontend/dist").not_found_service(ServeFile::new("frontend/dist/index.html"));

  let router = Router::new()
    .route("/images", get(get_images))
    .route("/images", post(new_image))
    .route("/images", delete(delete_all))
    .route("/images/{filename}", get(get_image_file))
    .route("/images/{filename}/invert", post(invert))
    .route("/images/{filename}/threshold", post(threshold))
    .route("/images/{filename}", delete(delete_image))
    .route("/printer", get(get_printer))
    .route("/printer/refresh", get(refresh_printer))
    .route("/preview", get(get_preview))
    .route("/print", post(print_label))
    .with_state(state)
    .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
    .fallback_service(serve_dir);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

  tracing::info!("Starting server on port 3000");
  axum::serve(listener, router).await?;

  Ok(())
}

async fn get_images(State(state): State<AppState>) -> Json<Vec<ImageWithId>> {
  Json(state.images())
}

async fn get_preview(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
  let image = Image::join(
    state
      .images()
      .iter()
      .map(|image| &image.image.processed_file),
    state.printer.lock().await.status(),
  );

  let buf = Vec::new();

  let mut cursor = Cursor::new(buf);
  match image.write_to(&mut cursor, ImageFormat::Png) {
    Ok(_) => (),
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)?,
  };

  let buf = cursor.into_inner();

  Ok(([(header::CONTENT_TYPE, "image/png")], buf))
}

#[axum::debug_handler]
async fn print_label(State(state): State<AppState>) -> Result<(), StatusCode> {
  let image = Image::render(
    state
      .images()
      .iter()
      .map(|image| &image.image.processed_file),
    state.printer.lock().await.status(),
  );

  let printer = state.printer.lock().await;

  let res = printer.print(image.into()).await;

  res.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  state.delete_all();

  Ok(())
}

#[derive(Debug, Clone, serde::Deserialize)]
struct PostInvert {
  invert: bool,
}

async fn invert(
  State(state): State<AppState>,
  Path(id): Path<Uuid>,
  Json(body): Json<PostInvert>,
) -> Result<(), StatusCode> {
  let status = state.printer.lock().await.status();

  state
    .with_image_mut(id, |image: &mut Image| {
      image.set_inverted(body.invert, status.clone());
    })
    .await;

  if true {
    Ok(())
  } else {
    Err(StatusCode::NOT_FOUND)
  }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct PostThreshold {
  threshold: u8,
}

async fn threshold(
  State(state): State<AppState>,
  Path(id): Path<Uuid>,
  Json(body): Json<PostThreshold>,
) -> Result<(), StatusCode> {
  let status = state.printer.lock().await.status();

  state
    .with_image_mut(id, |image: &mut Image| {
      image.set_threshold(body.threshold, status.clone());
    })
    .await;

  if true {
    Ok(())
  } else {
    Err(StatusCode::NOT_FOUND)
  }
}

async fn get_image_file(
  State(state): State<AppState>,
  Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
  let image = state.get_image(id);

  if let Some(image) = image {
    let buf = Vec::new();

    let mut cursor = Cursor::new(buf);
    match image.processed_file.write_to(&mut cursor, ImageFormat::Png) {
      Ok(_) => (),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    let buf = cursor.into_inner();

    Ok(([(header::CONTENT_TYPE, "image/png")], buf))
  } else {
    Err(StatusCode::NOT_FOUND)?
  }
}

async fn new_image(
  State(state): State<AppState>,
  multipart: Multipart,
) -> Result<Json<ImageWithId>, ()> {
  let printer_ty = state.printer.lock().await.ty();
  let status = state.printer.lock().await.status();

  let image = Image::from_multipart(printer_ty, status, multipart).await?;

  let id = state.add_image(image.clone());

  Ok(Json(ImageWithId { id, image }))
}

async fn delete_all(State(state): State<AppState>) {
  state.delete_all();
}

async fn delete_image(State(state): State<AppState>, Path(id): Path<Uuid>) -> StatusCode {
  let out = state.delete_image(id);

  if out.is_some() {
    StatusCode::OK
  } else {
    StatusCode::NOT_FOUND
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct PrinterInfo {
  ty: PrinterType,
  dpi: u32,
  max_px: u32,
  media_type: MediaType,
  media_width: TapeSize,
  tape_color: TapeColor,
  text_color: TextColor,
}

async fn get_printer(State(state): State<AppState>) -> Json<PrinterInfo> {
  let ty = state.printer.lock().await.ty();
  let info = ty.info();
  let status = state.printer.lock().await.status();

  let info = PrinterInfo {
    ty,
    dpi: info.dpi,
    max_px: info.max_px,
    media_type: status.media_type,
    media_width: status.media_width,
    tape_color: status.tape_color,
    text_color: status.text_color,
  };

  Json(info)
}

#[axum::debug_handler]
async fn refresh_printer(
  State(mut state): State<AppState>,
) -> Result<Json<PrinterInfo>, StatusCode> {
  state.reload_printer().await.map_err(|e| {
    dbg!(e);
    StatusCode::INTERNAL_SERVER_ERROR
  })?;

  Ok(get_printer(State(state)).await)
}
