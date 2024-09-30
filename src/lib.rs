//! # egui_dialogs
//!
//! Platform-agnostic, customizable dialogs for egui library.
//!
//! ## Quick start
//!
//! ### Run the example
//!
//! ```bash
//! # cd into the crate directory
//! cargo run --example dialogs
//! ```
//!
//! ### Basic usage
//!
//! Install the crate:
//!
//! ```bash
//! cargo add egui_dialogs
//! ```
//!
//! Then add a `Dialogs` field to your `App` struct:
//!
//! ```
//! use egui_dialogs::Dialogs;
//!
//! pub struct MyApp<'a> {
//!     // ... your other app states
//!     dialogs: Dialogs<'a>,
//! }
//! ```
//!
//! Then somewhere in your `App::update` function:
//!
//! ```
//! # use egui_dialogs::Dialogs;
//! #
//! # pub struct MyApp<'a> {
//! #     // ... your other app states
//! #     dialogs: Dialogs<'a>,
//! # }
//! 
//! impl MyApp<'_> {
//!     // ... your other app logic
//!
//!     pub fn update(&mut self, ctx: &egui::Context) {
//!         self.dialogs.show(ctx);
//!
//!         // ... your other rendering logic
//!     }
//! }
//! ```
//!
//! And when you want to show a dialog:
//!
//! ```
//! # use egui_dialogs::Dialogs;
//! #
//! # pub struct MyApp<'a> {
//! #     // ... your other app states
//! #     dialogs: Dialogs<'a>,
//! # }
//! #
//! # impl MyApp<'_> {
//! #     // ... your other app logic
//! #
//! #     pub fn update(&mut self, ctx: &egui::Context) {
//! #         self.dialogs.show(ctx);
//! #
//! #         // ... your other rendering logic
//! self.dialogs.info("Information", "This is an info dialog");
//! #     }
//! # }
//! ```
//!
//! ### Handle reply
//! 
//! #### Using callback functions
//!
//! Use `DialogDetails` struct to build
//! a dialog with custom attributes.
//!
//! The following is an example to comfirm a window close request:
//!
//! ```
//! use std::{cell::RefCell, rc::Rc};
//! 
//! use egui_dialogs::{DialogDetails, StandardReply};
//! 
//! # use egui_dialogs::Dialogs;
//! #
//! # pub struct MyApp<'a> {
//! #     // ... your other app states
//! #     dialogs: Dialogs<'a>,
//! // in your app state
//! pub allow_to_close: Rc<RefCell<bool>>,
//! // and initialize it with false
//! 
//! # }
//! #
//! # impl MyApp<'_> {
//! #     // ... your other app logic
//! #
//! #     pub fn update(&mut self, ctx: &egui::Context) {
//! #         self.dialogs.show(ctx);
//! #
//! #         // ... your other rendering logic
//! #
//! // when received a close request in the update function
//! if ctx.input(|i| i.viewport().close_requested()) {
//!   let ctx = ctx.clone();
//!   let allow_to_close = Rc::clone(&self.allow_to_close);
//!
//!   if *allow_to_close.borrow() {
//!       // run your close logic
//!   } else {
//!       // build and show a confirm dialog
//!       DialogDetails::confirm("Close", "Are you sure you want to close the window?")
//!           .on_reply(move |res| {
//!               if res == StandardReply::Ok {
//!                   *allow_to_close.borrow_mut() = true;
//!                   ctx.send_viewport_cmd(egui::ViewportCommand::Close);
//!               }
//!           })
//!           .show(&mut self.dialogs);
//!   }
//! }
//! #
//! #     }
//! # }
//! ```
//! 
//! #### Using IDs
//!
//! As rust compiler thinks that your dialog callbacks may be called at any time, it limits your callback from visiting your app states.
//!
//! To avoid filling your app state struct with `Rc` and `RefCell`, you can specify an ID for your dialog and use it to identify the dialog reply when a dialog is closed:
//!
//! ```rust
//! use egui::Id;
//! use egui_dialogs::{DialogDetails, StandardReply};
//!
//! # use egui_dialogs::Dialogs;
//! #
//! # pub struct MyApp<'a> {
//! #     // ... your other app states
//! #     dialogs: Dialogs<'a>,
//! // in your app state
//! pub allow_to_close: bool,
//! // and initialize it with false
//! 
//! # }
//! #
//! # impl MyApp<'_> {
//! #     // ... your other app logic
//! #
//! #     pub fn update(&mut self, ctx: &egui::Context) {
//! #         self.dialogs.show(ctx);
//! #
//! #         // ... your other rendering logic
//! #
//! // when received a close request in the update function
//! // define an ID for your dialog
//! const CLOSE_CONFIRM_DIALOG_ID: &str = "close_confirm_dialog";
//!
//! // in your update function
//! if let Some(res) = self.dialogs.show(ctx) {
//!     // handle reply from close confirmation dialog
//!     if res.is_reply_of(CLOSE_CONFIRM_DIALOG_ID) {
//!         match res.reply() {
//!             Ok(StandardReply::Yes) => {
//!                 self.allow_to_close = true;
//!                 ctx.send_viewport_cmd(egui::ViewportCommand::Close);
//!             },
//!             _ => {},
//!         }
//!     }
//! }
//!
//! // when you want to show the dialog
//! if ctx.input(|i| i.viewport().close_requested()) {
//!     if !self.allow_to_close {
//!         ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
//!         DialogDetails::confirm("Close", "Are you sure you want to close the window?")
//!             // specify the dialog ID
//!             .with_id(CLOSE_CONFIRM_DIALOG_ID)
//!             .show_if_absent(&mut self.dialogs);
//!     }
//! }
//! #
//! #     }
//! # }
//! ```
//!
//! ## Customization
//!
//! ### Customize standard dialog
//!
//! You can show a customized dialog based on the standard dialogs:
//!
//! ```
//! use egui::include_image;
//! use egui_dialogs::{DialogDetails, StandardDialog, StandardReply};
//!
//! # use egui_dialogs::Dialogs;
//! #
//! # pub struct MyApp<'a> {
//! #     // ... your other app states
//! #     dialogs: Dialogs<'a>,
//! # }
//! #
//! # impl MyApp<'_> {
//! #     // ... your other app logic
//! #
//! #     pub fn update(&mut self, ctx: &egui::Context) {
//! #         self.dialogs.show(ctx);
//! #
//! #         // ... your other rendering logic
//! let standard_dialog = StandardDialog::info("Information", "Now you can customize the dialog!")
//!     .buttons(vec![
//!         // use the standard buttons
//!         StandardReply::Yes.into(),
//!         // or add custom buttons with specific replies
//!         ("What?".into(), StandardReply::No)
//!     ])
//!     .image(include_image!("assets/info.svg"));
//!
//! DialogDetails::new(standard_dialog)
//!     .on_reply(|res| {
//!         match res {
//!             StandardReply::Yes => println!("That's great!"),
//!             StandardReply::No => println!("Emm...maybe you can try to see the example?"),
//!             _ => panic!("I've never added such a reply!")
//!         }
//!     })
//!     .show(&mut self.dialogs);
//! #     }
//! # }
//!
//! ```
//!
//! ### Customize dialog appearance and behavior
//!
//! To show a completely customized dialog, you can first design your dialog state struct like this:
//!
//! ```
//! pub struct NameConfirmDialog {
//!     name: String,
//! }
//! ```
//!
//! Then implement the `Dialog` trait to implement dialog logic
//! with a generic type parameter to specify the dialog reply type:
//!
//! ```
//! # pub struct NameConfirmDialog {
//! #     name: String,
//! # }
//! #
//! use egui_dialogs::{dialog_window, Dialog, DialogContext};
//! 
//! impl Dialog<String> for NameConfirmDialog {
//!   fn show(&mut self, ctx: &egui::Context, dctx: &DialogContext) -> Option<String> {
//!     // return None if the user hasn't replied
//!     let mut res = None;
//!
//!     // draw the dialog
//!     dialog_window(ctx, dctx, "Confirm name")
//!       .show(ctx, |ui| {
//!         ui.label("What's your name: ");
//!         ui.text_edit_singleline(&mut self.name);
//!         if ui.button("Done").clicked() {
//!           // set the reply and end the dialog
//!           res = Some(self.name.clone());
//!         }
//!       });
//!
//!     res
//!   }
//! }
//! ```
//!
//! The `dialog_window` function is a helper function
//! to draw a suggested dialog window with a title.
//!
//! Now you can show your customized dialog:
//!
//! ```
//! # pub struct NameConfirmDialog {
//! #     name: String,
//! # }
//! #
//! # use egui_dialogs::{dialog_window, Dialog, DialogDetails, DialogContext};
//! # 
//! # impl Dialog<String> for NameConfirmDialog {
//! #   fn show(&mut self, ctx: &egui::Context, dctx: &DialogContext) -> Option<String> {
//! #     // return None if the user hasn't replied
//! #     let mut res = None;
//! #
//! #     // draw the dialog
//! #     dialog_window(ctx, dctx, "Confirm name")
//! #       .show(ctx, |ui| {
//! #         ui.label("What's your name: ");
//! #         ui.text_edit_singleline(&mut self.name);
//! #         if ui.button("Done").clicked() {
//! #           // set the reply and end the dialog
//! #           res = Some(self.name.clone());
//! #         }
//! #       });
//! #
//! #     res
//! #   }
//! # }
//! # use egui_dialogs::Dialogs;
//! #
//! # pub struct MyApp<'a> {
//! #     // ... your other app states
//! #     dialogs: Dialogs<'a>,
//! # }
//! #
//! # impl MyApp<'_> {
//! #     // ... your other app logic
//! #
//! #     pub fn update(&mut self, ctx: &egui::Context) {
//! #         self.dialogs.show(ctx);
//! #
//! #         // ... your other rendering logic
//! DialogDetails::new(NameConfirmDialog { name: "".into() })
//!     .on_reply(|res| {
//!         println!("Your name is {}", res);
//!     })
//!     .show(&mut self.dialogs);
//! #     }
//! # }
//! ```

mod dialog_details;
mod dialogs;
mod standard_dialog;

pub use dialog_details::*;
pub use dialogs::*;
pub use standard_dialog::*;
