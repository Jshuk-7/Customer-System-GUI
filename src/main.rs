extern crate native_windows_gui as nwg;

mod main_window;

use nwg::NativeUi;
use main_window::MainWindow;

fn main() {
    nwg::init().expect("Failed to init GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set font");
    let _ui = MainWindow::build_ui(
        Default::default()
    ).expect("Failed to build");
    nwg::dispatch_thread_events();
}