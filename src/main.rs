use eframe::egui;
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Enlivl TextEditor"),
        ..Default::default()
    };

    eframe::run_native(
        "Enlivl TextEditor",
        options,
        Box::new(|_cc| Box::new(NotepadApp::default())),
    )
}

struct NotepadApp {
    content: String,
    current_file: Option<PathBuf>,
    word_wrap: bool,
    status_msg: String,
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            content: String::new(),
            current_file: None,
            word_wrap: true,
            status_msg: String::from("Ready"),
        }
    }
}

impl NotepadApp {
    fn new_file(&mut self) {
        self.content.clear();
        self.current_file = None;
        self.status_msg = String::from("New file created");
    }

    fn open_file(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Text Files", &["txt", "md", "rs", "log"])
            .pick_file()
        {
            match fs::read_to_string(&path) {
                Ok(text) => {
                    self.content = text;
                    self.status_msg = format!("Opened: {}", path.display());
                    self.current_file = Some(path);
                }
                Err(err) => {
                    self.status_msg = format!("Failed to open file: {}", err);
                }
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(ref path) = self.current_file {
            match fs::write(path, &self.content) {
                Ok(_) => self.status_msg = format!("Saved: {}", path.display()),
                Err(err) => self.status_msg = format!("Failed to save: {}", err),
            }
        } else {
            self.save_file_as();
        }
    }

    fn save_file_as(&mut self) {
        if let Some(path) = FileDialog::new().add_filter("Text File", &["txt"]).save_file() {
            match fs::write(&path, &self.content) {
                Ok(_) => {
                    self.status_msg = format!("Saved as: {}", path.display());
                    self.current_file = Some(path);
                }
                Err(err) => self.status_msg = format!("Failed to save file: {}", err),
            }
        }
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top Menu Bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.new_file();
                        ui.close_menu();
                    }
                    if ui.button("Open...").clicked() {
                        self.open_file();
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        self.save_file();
                        ui.close_menu();
                    }
                    if ui.button("Save As...").clicked() {
                        self.save_file_as();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.word_wrap, "Word Wrap");
                });
            });
        });

        // Bottom Status Bar
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let lines = self.content.lines().count();
                let chars = self.content.chars().count();
                ui.label(&self.status_msg);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Lines: {} | Chars: {}", lines, chars));
                });
            });
        });

// Central Text Area
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let width = if self.word_wrap {
                    ui.available_width()
                } else {
                    f32::INFINITY
                };

                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.content)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_width(width),
                );
            });
        });
    }
}
