use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "cc.cinea.statusBarColor";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_status_bar_color);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<StatusBarColor<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "StatusBarColorPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_status_bar_color)?;
  Ok(StatusBarColor(handle))
}

/// Access to the status-bar-color APIs.
pub struct StatusBarColor<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> StatusBarColor<R> {
  pub fn set_color(&self, payload: SetColorRequest) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("setColor", payload)
      .map_err(Into::into)
  }
}
