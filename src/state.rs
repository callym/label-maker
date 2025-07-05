use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

use ptouch_rs::Printer;
use tokio::sync::Mutex;
use tracing::Level;
use uuid::Uuid;

use crate::Image;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  PTouch(#[from] ptouch_rs::Error),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageWithId {
  pub id: Uuid,
  #[serde(flatten)]
  pub image: Image,
}

#[derive(Clone)]
pub struct AppState {
  pub printer: Arc<Mutex<Printer>>,
  pub images: Arc<RwLock<HashMap<Uuid, Image>>>,
}

impl AppState {
  pub async fn new() -> Result<Self, ptouch_rs::Error> {
    let span = tracing::span!(Level::INFO, "Opening printer").entered();
    let printer = ptouch_rs::Printer::open().await?;
    drop(span);

    Ok(Self {
      printer: Arc::new(Mutex::new(printer)),
      images: Arc::new(RwLock::new(HashMap::new())),
    })
  }

  pub async fn reload_printer(&mut self) -> Result<(), Error> {
    self.printer.lock().await.reload_status().await?;

    Ok(())
  }

  pub fn images(&self) -> Vec<ImageWithId> {
    let mut images = self
      .images
      .read()
      .unwrap()
      .clone()
      .into_iter()
      .map(|(id, image)| ImageWithId { id, image })
      .collect::<Vec<_>>();

    images.sort_by_key(|img| img.id);

    images
  }

  pub fn get_image(&self, id: Uuid) -> Option<Image> {
    self.images.read().unwrap().get(&id).cloned()
  }

  pub async fn with_image_mut(&self, id: Uuid, fun: impl Fn(&mut Image)) -> Option<()> {
    let mut images = self.images.write().unwrap();
    let image = images.get_mut(&id);

    if let Some(image) = image {
      fun(image);

      Some(())
    } else {
      None
    }
  }

  pub fn add_image(&self, image: Image) -> Uuid {
    let id = Uuid::now_v7();
    self.images.write().unwrap().insert(id, image);
    id
  }

  pub fn delete_all(&self) {
    self.images.write().unwrap().clear();
  }

  pub fn delete_image(&self, id: Uuid) -> Option<Image> {
    self.images.write().unwrap().remove(&id)
  }
}
