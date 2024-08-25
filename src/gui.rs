// src/gui.rs

use crate::steganography;

use eframe::egui;
use rfd::FileDialog;

pub struct ShadesGui {
    pub embed_box: bool,
    pub extract_box: bool,
    pub input: String,
    pub output: String,
    pub password: String,
    pub selected_file: Option<String>,
    pub status: String,
}

impl Default for ShadesGui {
    fn default() -> Self {
        Self {
            embed_box: false,
            extract_box: false,
            input: String::new(),
            output: String::new(),
            password: String::new(),
            selected_file: None,
            status: String::new(),
        }
    }
}

impl eframe::App for ShadesGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Center everything in a vertical layout
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Some padding
                ui.spacing_mut().item_spacing = egui::vec2(20.0, 20.0);
                ui.spacing_mut().button_padding = egui::vec2(20.0, 10.0);

                ui.horizontal(|ui| {
                    if ui.checkbox(&mut self.embed_box, "Embed").clicked() {
                        if self.embed_box {
                            self.extract_box = false; // Uncheck 'Extract' if 'Embed' is checked
                        }
                    }

                    if ui.checkbox(&mut self.extract_box, "Extract").clicked() {
                        if self.extract_box {
                            self.embed_box = false; // Uncheck 'Embed' if 'Extract' is checked
                        }
                    }
                });

                // Status message
                if self.embed_box || self.extract_box {
                    ui.horizontal(|ui| {
                        ui.heading(&self.status);
                    });
                }

                if self.embed_box {
                    // 'Embed' mode UI elements

                    if ui.button("Select a file").clicked() {
                        if let Some(file) = FileDialog::new().pick_file() {
                            self.selected_file = Some(file.display().to_string());
                        }
                    }
                    
                    if let Some(file) = &self.selected_file {
                        ui.label(format!("Selected file: {}", file));
                    }

                    ui.label("Enter your data: ");
                    ui.text_edit_multiline(&mut self.input);

                    ui.label("Password: ");
                    ui.add(egui::TextEdit::singleline(&mut self.password).password(true));

                    if ui.button("Run").clicked() {
                        if let Some(ref file_path) = self.selected_file {
                            match steganography::embed_encrypted_data(
                                file_path,
                                &self.input,
                                &self.password,
                            ) {
                                Ok(_) => self.status = String::from("Data embedded in image successfully."),
                                Err(e) => self.status = format!("Failed to embed data: {}", e),
                            };
                        } else {
                            self.status = String::from("Please select a file first.");
                        }
                    }
                } else if self.extract_box {
                    // 'Extract' mode UI elements

                    if ui.button("Select a file").clicked() {
                        if let Some(file) = FileDialog::new().pick_file() {
                            self.selected_file = Some(file.display().to_string());
                        }
                    }
                    
                    if let Some(file) = &self.selected_file {
                        ui.label(format!("Selected file: {}", file));
                    }

                    ui.label("Password: ");
                    ui.add(egui::TextEdit::singleline(&mut self.password).password(true));

                    if ui.button("Run").clicked() {
                        if let Some(ref file_path) = self.selected_file {
                            match steganography::extract_encrypted_data(file_path, &self.password) {
                                Ok(decrypted_data) => {
                                    self.output = decrypted_data;
                                    self.input.clear();
                                    self.status = String::from("Data extracted successfully.");
                                }
                                Err(e) => self.status = format!("Failed to extract data: {}", e),
                            };
                        } else {
                            self.status = String::from("Please select a file first.");
                        }
                    }

                    // Output box for displaying extracted data
                    ui.label("Output:");
                    ui.text_edit_multiline(&mut self.output);
                    
                } else {
                    // If no checkbox is selected, the 'start page' is shown and temporary data cleared

                    self.input.clear();
                    self.output.clear();
                    self.status.clear();
                    self.password.clear();
                    self.selected_file = None;

                    ui.heading("Welcome to Shades.");
                    ui.label("Please select either 'Embed' or 'Extract' to continue.");
                    ui.label("Use 'Embed' to hide data within a PNG image, or 'Extract' to retrieve the hidden data.");
                    ui.label("The data is encrypted with state-of-the-art AES-256 cryptography for maximum security.");
                    ui.label("This application is free and open-source. You can view the code here:");
                    ui.hyperlink("https://github.com/arsquid/shades/");
                }
            });
        });
    }
}
