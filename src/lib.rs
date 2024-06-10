#![cfg(mobile)]

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

mod mobile;

mod models;

use mobile::Recaptcha;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the recaptcha APIs.
pub trait RecaptchaExt<R: Runtime> {
  fn recaptcha(&self) -> &Recaptcha<R>;
}

impl<R: Runtime, T: Manager<R>> RecaptchaExt<R> for T {
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
