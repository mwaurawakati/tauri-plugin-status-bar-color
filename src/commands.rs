use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::StatusBarColorExt;

#[command]
pub(crate) async fn set_color<R: Runtime>(
    app: AppHandle<R>,
    payload: SetColorRequest,
) -> Result<()> {
    app.status_bar_color().set_color(payload)
}
