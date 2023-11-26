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

// ▼ List 5-25
// Function to handle additional drawing (circles in this case)
fn plot(ui: &mut egui::Ui) {
    // Define the position and radius for the first filled circle
    let pos_1 = egui::Pos2::new(100.0, 100.0);
    let radius_1 = 50.0;

    // Draw the first filled circle with a red color
    ui.painter()
        .circle_filled(pos_1, radius_1, egui::Color32::RED);

    // Define the position, radius, and stroke for the second stroked circle
    let pos_2 = egui::Pos2::new(150.0, 150.0);
    let radius_2 = 50.0;
    let stroke_2 = egui::Stroke::from((10.0, egui::Color32::GREEN));

    // Draw the second stroked circle with a green border
    ui.painter().circle_stroke(pos_2, radius_2, stroke_2);
}
