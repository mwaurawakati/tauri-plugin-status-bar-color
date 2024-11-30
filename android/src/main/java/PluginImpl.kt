package cc.cinea.statusBarColor

import android.app.Activity
import android.graphics.Color
import android.os.Build
import android.view.View
import android.view.WindowInsetsController

class PluginImpl {
    fun setColor(hex: String, activity: Activity) {
        val color = Color.parseColor(hex)
        val isLightColor = isColorLight(color)

        // Set the status bar color
        activity.window.statusBarColor = color

        // Set the navigation bar color
        activity.window.navigationBarColor = color

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            // Adjust system bar appearance for light/dark content
            val appearance = if (isLightColor) {
                WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS or WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS
            } else {
                0
            }
            activity.window.insetsController?.setSystemBarsAppearance(
                appearance,
                WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS or WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS
            )
        } else if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            @Suppress("DEPRECATION")
            activity.window.decorView.systemUiVisibility = if (isLightColor) {
                View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR or View.SYSTEM_UI_FLAG_LIGHT_NAVIGATION_BAR
            } else {
                0
            }
        }
    }

    private fun isColorLight(color: Int): Boolean {
        val red = Color.red(color)
        val green = Color.green(color)
        val blue = Color.blue(color)
        val brightness = (0.299 * red + 0.587 * green + 0.114 * blue) / 255
        return brightness > 0.5
    }
}
