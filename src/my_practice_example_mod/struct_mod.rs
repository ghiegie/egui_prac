use eframe::egui;

// create MyApp struct (type) that has the default attrib
#[derive(Default)]
pub struct MyApp {
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
}

// implement App trait for MyApp type
impl eframe::App for MyApp {
    // takes a mutable ref to self
    // a context that takes an immut ref to Context struct (type)
    // a frame which will not be used that takes a mutable reference to Frame struct (type)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // create CentralPanel struct, set to default
        // then show it. the show takes the context and a closure
        // the closure takes a mutable ref to Ui struct (type)
        egui::CentralPanel::default().show(ctx, |_ui| {
        });
    }
}
