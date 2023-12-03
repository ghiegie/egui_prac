use eframe::egui;

use super::struct_mod::MyApp;

pub fn run_app() -> Result<(), eframe::Error> {
    env_logger::init(); //log to stderr (if you run with RUST_LOG=debug)

    // create nativeoptions struct
    let options = eframe::NativeOptions {
        // viewport field requires a viewportbuilder struct
        // viewportbuilder struct build new viewport (window)
        // default creates the default viewportbuilder struct
        // with_inner_size sets the start size of window
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // starts the native desktop app
    // app name is given, the options struct is sent to native_options
    // app creator > create a box referencing the value returned by the closure
    // the closure takes cc which will not be used
    // then returns a box of type MyApp(?) and set its values to default
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}
