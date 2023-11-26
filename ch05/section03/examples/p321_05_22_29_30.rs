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

// ▼リスト5-29
// Structure representing the Egui application
struct MyEguiApp {
    // Vector to store the positions of clicked points
    click_pos: Vec<egui::Pos2>,
}

// Implementation of default values for MyEguiApp
impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp { click_pos: vec![] }
    }
}

// Implementation of methods for the MyEguiApp structure
impl MyEguiApp {
    // Constructor for creating a new instance of MyEguiApp
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// ▼リスト5-30
// Implementation of the eframe::App trait for MyEguiApp
impl eframe::App for MyEguiApp {
    // Update method for handling updates in the Egui application
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Show a central panel in the Egui interface with a heading
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            // Allocate a response area for click events
            let resp = ui.allocate_response(egui::vec2(400.0, 300.0), egui::Sense::click());

            // Check if the response was clicked
            if resp.clicked() {
                // Get the position of the click and add it to the vector
                let p = resp.interact_pointer_pos().unwrap();
                self.click_pos.push(p);
            }

            // Call the plot function to handle the drawing
            plot(ui, &self.click_pos);
        });
    }
}

// Function to draw circles at specified positions
fn plot(ui: &mut egui::Ui, pos: &Vec<egui::Pos2>) {
    for p in pos {
        // Draw filled circles at each specified position
        ui.painter().circle_filled(
            *p,
            25.0,
            egui::Color32::from_rgba_premultiplied(255, 0, 0, 100),
        );
    }
}
