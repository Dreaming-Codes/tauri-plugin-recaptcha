#![cfg(mobile)]

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

mod mobile;

mod error;
mod models;

pub use error::{Error, Result};

use mobile::Recaptcha;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the recaptcha APIs.
pub trait RecaptchaExt<R: Runtime> {
  fn recaptcha(&self) -> &Recaptcha<R>;
}

impl<R: Runtime, T: Manager<R>> crate::RecaptchaExt<R> for T {
  fn recaptcha(&self) -> &Recaptcha<R> {
    self.state::<Recaptcha<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("recaptcha")
    .setup(|app, api| {
      let recaptcha = mobile::init(app, api)?;
      app.manage(recaptcha);

      Ok(())
    })
    .build()
}
