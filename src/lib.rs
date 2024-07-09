use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::StatusBarColor;
#[cfg(mobile)]
use mobile::StatusBarColor;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the status-bar-color APIs.
pub trait StatusBarColorExt<R: Runtime> {
  fn status_bar_color(&self) -> &StatusBarColor<R>;
}

impl<R: Runtime, T: Manager<R>> crate::StatusBarColorExt<R> for T {
  fn status_bar_color(&self) -> &StatusBarColor<R> {
    self.state::<StatusBarColor<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("status-bar-color")
    .invoke_handler(tauri::generate_handler![commands::set_color])
    .setup(|app, api| {
      #[cfg(mobile)]
      let status_bar_color = mobile::init(app, api)?;
      #[cfg(desktop)]
      let status_bar_color = desktop::init(app, api)?;
      app.manage(status_bar_color);
      Ok(())
    })
    .build()
}
