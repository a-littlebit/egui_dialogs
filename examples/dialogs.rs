use egui::{vec2, CentralPanel, Context};
use egui_dialogs::{dialog_window, Dialog, DialogContext, DialogDetails, Dialogs, StandardReply};

fn main() -> Result<(), eframe::Error> {
    // Create native window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
                      .with_inner_size([400.0, 300.0]),
        centered: true,
        ..Default::default()
    };

    // Run native app
    eframe::run_native(
        "egui dialogs example", // window title
        options, // viewport options
        Box::new(|cc| {
            // the crate uses egui_extra for standard icon loading
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // we transtale standard button texts into native language
            // according to system locale.
            // setup a font to support non-latin characters
            // 
            // deprecated for crates.io uploading package size limit
            // setup_custom_fonts(&cc.egui_ctx);

            // setup custom style
            setup_style(&cc.egui_ctx);
            
            // Create the app instance
            Ok(Box::new(DialogApp::new(cc)))
        }),
    )
}

// deprecated for crates.io uploading package size limit
// fn setup_custom_fonts(ctx: &egui::Context) {
//     // Start with the default fonts (we will be adding to them rather than replacing them).
//     let mut fonts = egui::FontDefinitions::default();

//     // Install my own font (maybe supporting non-latin characters).
//     // .ttf and .otf files supported.
//     fonts.font_data.insert(
//         "my_font".to_owned(),
//         egui::FontData::from_static(include_bytes!(
//             "fonts/Ubuntu-Light.ttf"
//         )),
//     );

//     // Put my font first (highest priority) for proportional text:
//     fonts
//         .families
//         .entry(egui::FontFamily::Proportional)
//         .or_default()
//         .insert(0, "my_font".to_owned());

//     // Put my font as last fallback for monospace:
//     fonts
//         .families
//         .entry(egui::FontFamily::Monospace)
//         .or_default()
//         .insert(0, "my_font".to_owned());

//     // Tell egui to use these fonts:
//     ctx.set_fonts(fonts);
// }

fn setup_style(ctx: &Context) {
    ctx.style_mut(|s| {
        s.spacing.item_spacing = vec2(8., 12.);
        s.spacing.button_padding = vec2(8., 6.);

        s.animation_time = 0.3;
    });
}

struct DialogApp<'a> {
    dialogs: Dialogs<'a>,

    title: String,
    content: String,

    confirmed_name: String,
    allow_to_close: bool,
}

impl DialogApp<'_> {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            dialogs: Dialogs::default(),
            title: "egui dialogs".into(),
            content: "hello, world!".into(),
            confirmed_name: "ferris".into(),
            allow_to_close: false,
        }
    }
}

impl eframe::App for DialogApp<'_> {  
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        const CLOSE_CONFIRM_DIALOG_ID: &str = "close_confirm_dialog";
        const NAME_CONFIRM_DIALOG_ID: &str = "name_confirm_dialog";
        const NAME_INPUT_CONFIRM_DIALOG_ID: &str = "name_input_confirm_dialog";
        
        // Show dialogs and handle the reply if there is one
        if let Some(res) = self.dialogs.show(ctx) {
            if res.is_reply_of(CLOSE_CONFIRM_DIALOG_ID) {
                match res.reply() {
                    Ok(StandardReply::Yes) => {
                        self.allow_to_close = true;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    },
                    _ => {}
                }
            } else if res.is_reply_of(NAME_CONFIRM_DIALOG_ID) {
                match res.reply() {
                    Ok(StandardReply::No) => {
                        self.confirmed_name = "".into();
                    },
                    _ => {}
                }
            } else if res.is_reply_of(NAME_INPUT_CONFIRM_DIALOG_ID) {
                match res.reply() {
                    Ok(name) => {
                        self.confirmed_name = name;
                    },
                    _ => {}
                }
            }
        }

        if ctx.input(|i| i.viewport().close_requested()) {
            if !self.allow_to_close {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                DialogDetails::confirm("Close", "Are you sure you want to close the window?")
                    .with_id(CLOSE_CONFIRM_DIALOG_ID)
                    .show(&mut self.dialogs);
            }
        }
        
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui dialogs example");

            ui.horizontal(|ui| {
                ui.label("Title: ");
                ui.text_edit_singleline(&mut self.title);
            });

            ui.horizontal(|ui| {
                ui.label("Content: ");
                ui.text_edit_singleline(&mut self.content);
            });

            // Show standard dialogs
            ui.horizontal(|ui| {
                if ui.button("Show info dialog").clicked() {
                    self.dialogs.info(self.title.clone(), self.content.clone());
                }
                
                if ui.button("Show success dialog").clicked() {
                    self.dialogs.success(self.title.clone(), self.content.clone());
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Show warning dialog").clicked() {
                    self.dialogs.warning(self.title.clone(), self.content.clone());
                }
                
                if ui.button("Show error dialog").clicked() {
                    self.dialogs.error(self.title.clone(), self.content.clone());
                }
            });

            ui.horizontal(|ui| {
                // Show standard confirm dialog with id
                if ui.button("Show confirm dialog").clicked() {
                    DialogDetails::confirm(
                        "Confirm name", 
                        format!("Is your name {}?", self.confirmed_name)
                    )
                    .with_id(NAME_CONFIRM_DIALOG_ID)
                    .show(&mut self.dialogs);
                }

                // Show custom dialog with id
                if ui.button("Custom dialog").clicked() {
                    DialogDetails::new(NameConfirmDialog::new(self.confirmed_name.clone()))
                        .with_id(NAME_INPUT_CONFIRM_DIALOG_ID)
                        .show_if_absent(&mut self.dialogs);
                }
            });
            
            if !self.confirmed_name.is_empty() {
                ui.label(format!("Your name is {}", self.confirmed_name));
            }
        });
    }
}

// custom dialog for name confirmation
pub struct NameConfirmDialog {
    name: String,
}

impl NameConfirmDialog {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

// implement dialog logic
impl Dialog<String> for NameConfirmDialog {
    fn show(&mut self, ctx: &Context, dctx: &DialogContext) -> Option<String> {
        // return None if the user hasn't replied
        let mut res = None;

        // draw the dialog
        dialog_window(ctx, dctx, "Confirm name")
            .show(ctx, |ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
                if ui.button("Done").clicked() {
                    // set the reply and end the dialog
                    res = Some(self.name.clone());
                }
            });
            
        res
    }
}