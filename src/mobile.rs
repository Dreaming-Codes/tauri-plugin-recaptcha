use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};
use tauri::plugin::mobile::PluginInvokeError;

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.recaptcha";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_recaptcha);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> Result<Recaptcha<R>, PluginInvokeError> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "RecaptchaPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_recaptcha)?;
  Ok(Recaptcha(handle))
}

/// Access to the recaptcha APIs.
pub struct Recaptcha<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Recaptcha<R> {
  pub fn do_recaptcha_challenge(&self, payload: DoRecaptchaChallengeRequest) -> Result<DoRecaptchaChallengeResponse, PluginInvokeError> {
    self
      .0
      .run_mobile_plugin("doRecaptchaChallenge", payload)
  }
}
