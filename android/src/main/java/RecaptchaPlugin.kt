package com.plugin.recaptcha

import android.annotation.SuppressLint
import android.app.Activity
import android.graphics.Color
import android.view.ViewGroup
import android.webkit.JavascriptInterface
import android.webkit.WebView
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class DoRecaptchaChallengeArgs {
    lateinit var html: String
    lateinit var baseURL: String
}

@TauriPlugin
class RecaptchaPlugin(private val activity: Activity): Plugin(activity) {
    @SuppressLint("SetJavaScriptEnabled")
    @Command
    fun doRecaptchaChallenge(invoke: Invoke) {
        val args = invoke.parseArgs(DoRecaptchaChallengeArgs::class.java)

        val recaptchaWebview = WebView(activity);
        recaptchaWebview.setBackgroundColor(Color.TRANSPARENT)
        val layoutParams = ViewGroup.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.MATCH_PARENT)
        activity.addContentView(recaptchaWebview, layoutParams);

        class RecaptchaWebviewInterface {
            @JavascriptInterface
            fun onRecaptchaSuccess(token: String) {
                val ret = JSObject()
                ret.put("token", token)
                invoke.resolve(ret)
                activity.runOnUiThread {
                    recaptchaWebview.destroy()
                }
            }
        }

        recaptchaWebview.addJavascriptInterface(RecaptchaWebviewInterface(), "RecaptchaWebviewInterface");

        recaptchaWebview.settings.javaScriptEnabled = true
        recaptchaWebview.loadDataWithBaseURL(args.baseURL, args.html, "text/html", "UTF-8", null);
    }
}
