// ▼ List 5-4
// Main function to run the Egui application
fn main() {
    // Create native options with default settings
    let mut native_options = eframe::NativeOptions::default();

    // Set the default theme to Light
    native_options.default_theme = eframe::Theme::Light;

    // Run the native Egui application with the specified title, options, and app creation logic
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

// Default implementation of MyEguiApp structure
#[derive(Default)]
struct MyEguiApp {}

// Implementation of methods for the MyEguiApp structure
impl MyEguiApp {
    // Constructor for creating a new instance of MyEguiApp
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// Implementation of the eframe::App trait for MyEguiApp
// ▼ List 5-6
impl eframe::App for MyEguiApp {
    // Update method for handling updates in the Egui application
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Show a central panel in the Egui interface
        egui::CentralPanel::default().show(ctx, |ui| {
            // Display a heading in the panel
            ui.heading("Hello World!");

            // Create a rich text message with specific formatting
            let label_txt = egui::RichText::new("This is sample message.")
                .size(32.0) // Set font size to 32.0
                .color(egui::Color32::from_rgba_premultiplied(255, 0, 0, 100)) // Set color to red with some transparency
                .italics(); // Apply italics

            // Create a label with the rich text message
            let label = egui::Label::new(label_txt);

            // Add the label to the user interface
            ui.add(label);
        });
    }
}
