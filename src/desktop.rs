use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<StatusBarColor<R>> {
  Ok(StatusBarColor(app.clone()))
}

/// Access to the status-bar-color APIs.
pub struct StatusBarColor<R: Runtime>(AppHandle<R>);

impl<R: Runtime> StatusBarColor<R> {
  pub fn set_color(&self, payload: SetColorRequest) -> crate::Result<()> {
    Ok(())
  }
}
