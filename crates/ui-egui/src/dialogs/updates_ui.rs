//! Development updates are installation-managed; never contact or execute upstream releases.
use crate::SlideApp;
#[derive(Default)]
pub(crate) struct Runtime {
    _disabled: bool,
}
pub(crate) fn enable(_app: &mut SlideApp) {}
pub(crate) fn request_check(app: &mut SlideApp) {
    app.notify("OmaDeck updates are managed by your installation. Automatic updates are disabled.");
}
pub(crate) fn tick(app: &mut SlideApp, _ctx: &egui::Context) {
    let _ = &app.updates_rt;
}
pub(crate) fn show(_app: &mut SlideApp, _ctx: &egui::Context) {}
