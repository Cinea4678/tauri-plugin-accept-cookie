use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[cfg(target_os = "android")]
use tauri::Manager;

#[cfg(target_os = "android")]
mod mobile;

mod error;

pub use error::{Error, Result};

#[cfg(target_os = "android")]
use mobile::AcceptCookie;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the accept-cookie APIs.
#[cfg(target_os = "android")]
pub trait AcceptCookieExt<R: Runtime> {
    fn accept_cookie(&self) -> &AcceptCookie<R>;
}

#[cfg(target_os = "android")]
impl<R: Runtime, T: Manager<R>> crate::AcceptCookieExt<R> for T {
    fn accept_cookie(&self) -> &AcceptCookie<R> {
        self.state::<AcceptCookie<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("accept-cookie")
        .invoke_handler(tauri::generate_handler![])
        .setup(|_app, _api| {
            #[cfg(target_os = "android")]
            {
                let accept_cookie = mobile::init(_app, _api)?;
                _app.manage(accept_cookie);
                Ok(())
            }
            #[cfg(any(desktop, target_os = "ios"))]
            {
                Ok(())
            }
        })
        .build()
}
