package cc.cinea.acceptCookie

import android.app.Activity
import android.os.Build
import android.webkit.CookieManager
import android.webkit.WebView
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin

@TauriPlugin
class AllowCookiePlugin(private val activity: Activity): Plugin(activity) {
  override fun load(webView: WebView) {
    CookieManager.getInstance().setAcceptThirdPartyCookies(webView, true);
    super.load(webView)
  }
}
