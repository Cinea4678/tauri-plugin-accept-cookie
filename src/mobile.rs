use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "cc.cinea.acceptCookie";


// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<AcceptCookie<R>> {
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "AllowCookiePlugin")?;
  Ok(AcceptCookie(handle))
}


/// Access to the accept-cookie APIs.
pub struct AcceptCookie<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AcceptCookie<R> {
 
}
