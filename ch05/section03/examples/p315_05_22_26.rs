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

// ▼リスト5-26
fn plot(ui: &mut egui::Ui) {
    let pos_1 = egui::Pos2::new(50.0, 50.0);
    let pos_2 = egui::Pos2::new(200.0, 200.0);
    let stroke_1 = egui::Stroke::new(5.0, egui::Color32::RED);
    let stroke_2 = egui::Stroke::new(5.0, egui::Color32::GREEN);
    ui.painter()
        .vline(50.0, std::ops::RangeInclusive::new(50.0, 200.0), stroke_1);
    ui.painter()
        .hline(std::ops::RangeInclusive::new(50.0, 200.0), 50.0, stroke_1);
    ui.painter().line_segment([pos_1, pos_2], stroke_2);
}
