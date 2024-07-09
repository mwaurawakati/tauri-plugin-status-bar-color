import { invoke } from '@tauri-apps/api/core'

export async function setStatusBarColor(hexColor: string): Promise<null> {
  await invoke<{value?: string}>('plugin:status-bar-color|set_color', {
    payload: {
      hex: hexColor,
    },
  })
  return null
}
