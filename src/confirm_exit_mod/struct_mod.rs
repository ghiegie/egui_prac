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
        // the closure takes a mutable ref to Ui struct (type) then sets its heading field
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Try to close the window");
        });

        // call the input() of ctx
        // its arg is a closure, that takes an immut ref to i of type InputState
        // the closure calls viewport to return viewportinfo then is "told" to close using close_requested.
        // the input() returns a bool, if true then do this:
        if ctx.input(|i| i.viewport().close_requested()) {
            // if myapp is allowed to close, do this:
            if self.allowed_to_close {
            } else {
                // else
                // call the ctx to call send_viewport_cmd
                // send_viewport_cmd, sends a command to the current viewpoint
                // the command is viewportcommand enum of variant cancel close
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                // set tht myapp's field
                self.show_confirmation_dialog = true;
            }
        }

        // if myapp's field is true
        if self.show_confirmation_dialog {
            // create a window struct (type) using new()
            // it takes smothing that implements Into<WidgetText> (&str?)
            egui::Window::new("Do you want to quit")
                // then set collapsible to false
                .collapsible(false)
                // then set resizable to false
                .resizable(false)
                // show returns an option
                // None if window is not open
                // Some if window is collapsed (minimized?)
                // takes a ctx, and a closure
                // the closure takes a mut ref to ui of type Ui
                .show(ctx, |ui| {
                    // ui creates a button with text
                    // return a bool if clicked
                    // if true:
                    if ui.button("No").clicked() {
                        // set myapp's fields
                        self.show_confirmation_dialog = false;
                        self.allowed_to_close = false;
                    }

                    // another button, if it is clicked
                    if ui.button("Yes").clicked() {
                        // set myapp's fields
                        self.show_confirmation_dialog = false;
                        self.allowed_to_close = true;

                        // get a reference to the parent
                        // send a command to viewport
                        // command is viewportcommand enum of variant close
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
        }
    }
}
