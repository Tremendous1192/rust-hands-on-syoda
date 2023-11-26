// ▼ List 5-22
// Main function to run the Egui application
fn main() {
    // Create native options with default settings
    let mut native_options = eframe::NativeOptions::default();

    // Set the default theme to Light
    native_options.default_theme = eframe::Theme::Light;

    // Set the initial window size to 400x300 pixels
    native_options.initial_window_size = Some(egui::Vec2 { x: 400.0, y: 300.0 });

    // Run the native Egui application with the specified title, options, and app creation logic
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

// Default implementation for MyEguiApp
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
impl eframe::App for MyEguiApp {
    // Update method for handling updates in the Egui application
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Show a central panel in the Egui interface with a heading
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            // Call the plot function to handle the drawing
            plot(ui);
        });
    }
}

// Function to handle the plot drawing
fn plot(ui: &mut egui::Ui) {
    // Prepare drawing operations here
}
