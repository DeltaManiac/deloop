use std::collections::HashMap;

use eframe::{
    egui::{self, Color32},
    epi,
};

use crate::deloop;

pub struct App {
    root_dir: String,
    backed_up: bool,
    data: Option<HashMap<String, HashMap<String, Option<String>>>>,
    show_advanced: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            backed_up: false,
            root_dir: crate::BASE_PATH.clone().to_owned(),
            data: None,
            show_advanced: false,
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Deloop"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::auto_sized().show(ui, |ui| {
                ui.add(
                    egui::Label::new("DeLoop")
                        .heading()
                        .text_color(Color32::YELLOW),
                );
                ui.add(
                    egui::Label::new("A hacky tool to configure Loop Hero")
                        .text_color(Color32::WHITE),
                );
                ui.horizontal_wrapped(|ui| {
                    ui.add(egui::Label::new("Root Path:").text_color(Color32::WHITE));
                    ui.add(egui::Label::new(self.root_dir.clone()).text_color(Color32::LIGHT_BLUE));
                });
                ui.end_row();
                if self.backed_up {
                    ui.horizontal_wrapped(|ui| {
                        ui.add(egui::Label::new("Backed up at:").text_color(Color32::WHITE));
                        ui.add(egui::Label::new("Some time").text_color(Color32::LIGHT_BLUE));
                    });
                } else {
                    if ui.button("Backup Configuration File").clicked() {
                        self.backed_up = true;
                    }
                    ui.end_row();
                }
                if ui.button("Load").clicked() {
                    let c = format!("{}\\{}", self.root_dir.clone(), crate::INI_FILE.clone());
                    let p = std::path::Path::new(&c);
                    let data = deloop::load_ini(p).unwrap();
                    self.data = Some(data);
                }
                if self.show_advanced {
                    match &self.data {
                        Some(data) => {
                            for (header, props) in data {
                                ui.collapsing(header, |ui| {
                                    // egui::ScrollArea::auto_sized().show(ui, |ui| {
                                    egui::Grid::new("Properties")
                                        .striped(true)
                                        .min_col_width(10.0)
                                        .max_col_width(200.0)
                                        .show(ui, |ui| {
                                            for (k, v) in props {
                                                ui.label(k);
                                                ui.label(match v {
                                                    Some(v) => v.clone(),
                                                    _ => "".to_string(),
                                                });
                                                ui.end_row();
                                            }
                                        });
                                    // });
                                });
                            }
                        }
                        _ => (),
                    }
                } else {
                    //Todo add qyuick toggles
                }
            });
        });
    }
}
