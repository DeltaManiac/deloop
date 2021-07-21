use eframe::{egui, epi};

pub struct App {
    root_dir:String,
    backed_up: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { backed_up: false,root_dir:crate::BASE_PATH.clone().to_owned() }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Deloop"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });
    }
}
