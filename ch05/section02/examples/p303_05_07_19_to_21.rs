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

// ▼ List 5-19
// Structure representing the Egui application state
struct MyEguiApp {
    // Message and content fields for text input
    pub message: String,
    pub content: String,
}

// Default implementation for MyEguiApp
impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp {
            message: String::from("Hello"),
            content: String::from("This is content."),
        }
    }
}

// Implementation of methods for the MyEguiApp structure
impl MyEguiApp {
    // Constructor for creating a new instance of MyEguiApp
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// ▼ List 5-21
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

            // Create a message string with the input message
            let msg = format!("input:\"{}\"", self.message);
            let label_txt = egui::RichText::new(msg).font(egui::FontId::proportional(24.0));
            let label = egui::Label::new(label_txt);
            ui.add(label);

            // Add a separator for visual clarity
            ui.separator();

            // Create singleline text edit widget for message input
            let te_sl = egui::TextEdit::singleline(&mut self.message)
                .font(egui::FontId::proportional(20.0));
            let resp = ui.add(te_sl);

            // Check if the input has changed and update the content accordingly
            if resp.changed() {
                self.content = format!(
                    "{}\n{}",
                    self.message.to_uppercase(),
                    self.message.to_lowercase()
                );
            };

            // Create multiline text edit widget for content display
            let te_ml =
                egui::TextEdit::multiline(&mut self.content).font(egui::FontId::proportional(20.0));
            ui.add(te_ml);
        });
    }
}
