import SwiftRs
import Tauri
import UIKit
import WebKit

let MAGIC_NUMBER = 20030810;

extension UIColor {
    convenience init?(hex: String) {
        let r, g, b, a: CGFloat
        
        if hex.hasPrefix("#") {
            let start = hex.index(hex.startIndex, offsetBy: 1)
            let hexColor = String(hex[start...])
            
            if hexColor.count == 6 {
                let scanner = Scanner(string: hexColor)
                var hexNumber: UInt64 = 0
                
                if scanner.scanHexInt64(&hexNumber) {
                    r = CGFloat((hexNumber & 0xff0000) >> 16) / 255
                    g = CGFloat((hexNumber & 0x00ff00) >> 8) / 255
                    b = CGFloat(hexNumber & 0x0000ff) / 255
                    a = 1.0
                    
                    self.init(red: r, green: g, blue: b, alpha: a)
                    return
                }
            }
        }
        
        return nil
    }
}

class SetColorArgs: Decodable {
    let hex: String
}

class StatusBarColorPlugin: Plugin {
    override func load(webview: WKWebView) {
        
    }
    
    @objc public func setColor(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(SetColorArgs.self)
        
        let color = UIColor(hex: args.hex)
        
        // Change status bar color
        DispatchQueue.main.async {
            if let ws = UIApplication.shared.connectedScenes.first as? UIWindowScene {
                if let window = ws.windows.first {
                    if let existingStatusBar = window.viewWithTag(MAGIC_NUMBER) {
                        existingStatusBar.backgroundColor = color
                    } else {
                        if let statusBarManager = ws.statusBarManager {
                            let statusBar = UIView(frame: statusBarManager.statusBarFrame.insetBy(dx: 0, dy: -8))
                            statusBar.backgroundColor = color
                            statusBar.tag = MAGIC_NUMBER
                            window.addSubview(statusBar)
                        }
                    }
                    
                    // Change navigation bar (toolbar) color
                    UINavigationBar.appearance().barTintColor = color
                    UINavigationBar.appearance().tintColor = .white // Optional: sets icon and text color to white
                    UINavigationBar.appearance().titleTextAttributes = [
                        .foregroundColor: UIColor.white
                    ]
                }
            }
        }
        
        invoke.resolve()
    }
}

@_cdecl("init_plugin_status_bar_color")
func initPlugin() -> Plugin {
    return StatusBarColorPlugin()
}
