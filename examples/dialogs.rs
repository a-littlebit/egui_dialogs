use std::{cell::RefCell, rc::Rc};

use egui::{Align2, CentralPanel, Context, Window};
use egui_dialogs::{Dialog, DialogDetails, DialogUpdateInfo, Dialogs, StandardReply};

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

struct DialogApp<'a> {
  dialogs: Dialogs<'a>,
  allowed_to_close: Rc<RefCell<bool>>,

  title: String,
  content: String,

  confirmed_name: Rc<RefCell<String>>,
}

impl DialogApp<'_> {
  fn new(_cc: &eframe::CreationContext<'_>) -> Self {
    Self {
      dialogs: Dialogs::default(),
      allowed_to_close: Rc::new(RefCell::new(false)),
      title: "egui dialogs".into(),
      content: "hello, world!".into(),
      confirmed_name: Rc::new(RefCell::new("ferris".into())),
    }
  }
}

impl eframe::App for DialogApp<'_> {  
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // Show dialogs if there
    self.dialogs.show(ctx);
    
    CentralPanel::default().show(ctx, |ui| {
      ui.heading("egui dialogs example");

      // confirm close
      let allowed_to_close = self.allowed_to_close.clone();
      if ctx.input(|i| i.viewport().close_requested()) {
        if *self.allowed_to_close.borrow() {
            // do nothing - we will close
        } else {
          let ctx = ctx.clone();
          ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
          DialogDetails::confirm("Close", "Are you sure you want to close the window?")
            .on_reply(move |res| {
              if res == StandardReply::Yes {
                *allowed_to_close.borrow_mut() = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
              }
            })
            .show(&mut self.dialogs);
        }
      }

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
        // Show standard confirm dialog with custom reply handler
        if ui.button("Show confirm dialog").clicked() {
          let confirm = self.confirmed_name.clone();
          DialogDetails::confirm(
            "Confirm name", 
            format!("Is your name {}?", self.confirmed_name.borrow())
          )
            .on_reply(move |res| {
              if res == StandardReply::No {
                *confirm.borrow_mut() = "".into();
              }
            })
            .show(&mut self.dialogs);
        }

        // Show custom dialog with custom handler
        if ui.button("Custom dialog").clicked() {
          let confirm = self.confirmed_name.clone();
          DialogDetails::new(NameConfirmDialog::new(self.confirmed_name.borrow().clone()))
            .on_reply(move |res| {
              *confirm.borrow_mut() = res;
            })
            .show(&mut self.dialogs);
        }
      });
      
      if !self.confirmed_name.borrow().is_empty() {
        ui.label(format!("Your name is {}", self.confirmed_name.borrow()));
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
  fn show(&mut self, ctx: &Context, _: &DialogUpdateInfo) -> Option<String> {
    // return None if the user hasn't replied
    let mut res = None;

    // draw the dialog
    Window::new("Confirm name")
      .collapsible(false)
      .resizable(false)
      .pivot(Align2::CENTER_CENTER)
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