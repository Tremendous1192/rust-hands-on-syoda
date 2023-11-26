// ▼ List 5-7
// Main function to run the Egui application
fn main() {
    // Create native options with default settings
    let mut native_options = eframe::NativeOptions::default();

    // Set the default theme to Light
    native_options.default_theme = eframe::Theme::Light;

    // Set the initial window size to 400x200 pixels
    native_options.initial_window_size = Some(egui::Vec2 { x: 400.0, y: 200.0 });

    // Run the native Egui application with the specified title, options, and app creation logic
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

// Structure representing the Egui application state
struct MyEguiApp {
    pub value: usize,
}

// Default implementation for MyEguiApp
impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp { value: 0 }
    }
}

// Implementation of methods for the MyEguiApp structure
impl MyEguiApp {
    // Constructor for creating a new instance of MyEguiApp
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// Implementation of the eframe::App trait for MyEguiApp
impl eframe::App for MyEguiApp {
    // Update method for handling updates in the Egui application
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Show a central panel in the Egui interface
        egui::CentralPanel::default().show(ctx, |ui| {
            // Display a heading in the panel
            ui.heading("Hello World!");

            // Add spacing for better visual separation
            ui.spacing();

            // Display a message with the click count
            let msg = format!("Click {} times.", self.value);
            let label_txt = egui::RichText::new(msg).size(32.0);
            let label = egui::Label::new(label_txt);
            ui.add(label);

            // Add a separator for visual clarity
            ui.separator();

            // Create a button with the label "Click!"
            let btn_txt = egui::RichText::new("Click!").font(egui::FontId::proportional(24.0));
            let btn = egui::Button::new(btn_txt);

            // Add the button to the UI and check for clicks
            let resp = ui.add_sized(egui::Vec2 { x: 150.0, y: 40.0 }, btn);
            if resp.clicked() {
                // Increment the click count on button click
                self.value += 1;
            }
        });
    }
}
