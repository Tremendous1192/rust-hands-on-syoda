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

// ▼ List 5-10
// Enum to represent different radio button values
#[derive(PartialEq, Debug)]
enum RadioValue {
    First,
    Second,
    Third,
}

// ▼ List 5-11
// Structure representing the Egui application state
struct MyEguiApp {
    // Enum to hold the selected radio button value
    pub value: RadioValue,
}

// Default implementation for MyEguiApp
impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp {
            value: RadioValue::First,
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

// ▼ List 5-12
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

            // Display a message with the value of the selected radio button
            let msg = format!("checked = {:?}.", self.value);
            let label_txt = egui::RichText::new(msg).size(32.0);
            let label = egui::Label::new(label_txt);
            ui.add(label);

            // Add a separator for visual clarity
            ui.separator();

            // Create radio buttons with labels "First," "Second," and "Third"
            let label_1 = egui::RichText::new("First").size(24.0);
            let label_2 = egui::RichText::new("Second").size(24.0);
            let label_3 = egui::RichText::new("Third").size(24.0);
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.value, RadioValue::First, label_1);
                ui.radio_value(&mut self.value, RadioValue::Second, label_2);
                ui.radio_value(&mut self.value, RadioValue::Third, label_3);
            });
        });
    }
}
