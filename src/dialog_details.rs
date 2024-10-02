//! Define the `Dialog` trait which can be implemented to customize dialogs
//! and `DialogDetails` struct which can be used to show dialogs.

use std::any::Any;

use egui::{Color32, Id, WidgetText};

use crate::*;

/// Represents a dialog.
/// Implement this trait to customize dialogs.
/// 
/// # Example
/// ```
/// use egui_dialogs::{dialog_window, Dialog, DialogContext};
/// 
/// // custom dialog for name confirmation
/// pub struct NameConfirmDialog {
///   name: String,
/// }
///
/// impl NameConfirmDialog {
///   pub fn new(name: String) -> Self {
///     Self { name }
///   }
/// }
///
/// // implement dialog logic
/// impl Dialog<String> for NameConfirmDialog {
///   fn show(&mut self, ctx: &egui::Context, dctx: &DialogContext) -> Option<String> {
///     // return None if the user hasn't replied
///     let mut res = None;
///
///     // draw the dialog
///     dialog_window(ctx, dctx, "Confirm name")
///       .show(ctx, |ui| {
///         ui.label("Your name: ");
///         ui.text_edit_singleline(&mut self.name);
///         if ui.button("Done").clicked() {
///           // set the reply and end the dialog
///           res = Some(self.name.clone());
///         }
///       });
///
///     res
///   }
/// }
/// ```
pub trait Dialog<Reply> {
    /// Customized dialog rendering and response handling process.
    fn show(&mut self, ctx: &egui::Context, dctx: &DialogContext) -> Option<Reply>;
}

/// Details of a dialog to be shown and replied.
/// Used to build and show dialogs.
/// 
/// # Example
/// ```
/// use egui_dialogs::{DialogDetails, StandardReply};
///
/// # use egui_dialogs::Dialogs;
/// #
/// # pub struct MyApp<'a> {
/// #     // ... your other app states
/// #     dialogs: Dialogs<'a>,
/// # }
/// #
/// # impl MyApp<'_> {
/// #     // ... your other app logic
/// #
/// #     pub fn update(&mut self, ctx: &egui::Context) {
/// #         self.dialogs.show(ctx);
/// #
/// #         // ... your other rendering logic
/// // show a confirm dialog
/// // in your update function
/// DialogDetails::confirm("Confirm", "Are you sure you want to do this?")
///     .on_reply(|res| {
///         if res == StandardReply::Yes {
///             println!("User confirmed!");
///         }
///     })
///    .show(&mut self.dialogs);
/// #     }
/// # }
/// ```
pub struct DialogDetails<'a, Reply>
where Reply: 'a + Any {
    pub(crate) dialog: Box<dyn Dialog<Reply> + 'a>,
    pub(crate) mask: Option<Color32>,
    pub(crate) id: Option<Id>,
}

impl<'a, Reply> DialogDetails<'a, Reply>
where Reply: 'a + Any {
    #[inline]
    /// Create a `DialogDetails` struct with the specified dialog.
    pub fn new(dialog: impl Dialog<Reply> + 'a) -> Self
    {
        Self::new_dyn(Box::new(dialog))
    }
    
    pub fn new_dyn(dialog: Box<dyn Dialog<Reply> + 'a>) -> Self {
        Self {
            dialog,
            mask: Some(Color32::from_black_alpha(0x80)),
            id: None,
        }
    }

    #[inline]
    /// Return a new `DialogDetails` struct with the specified reply handler
    /// and a reply type mapped by the handler.
    pub fn on_reply<R: Any>(self, handler: impl FnOnce(Reply) -> R + 'a) -> DialogDetails<'a, R> {
        self.on_reply_dyn(Box::new(handler))
    }

    #[inline]
    /// dynamic version of [`Self::on_reply`]
    pub fn on_reply_dyn<R: Any>(self, handler: Box<dyn FnOnce(Reply) -> R + 'a>) -> DialogDetails<'a, R> {
        struct MappedDialog<'m, From, To> {
            dialog: Box<dyn Dialog<From> + 'm>,
            mapper: Option<Box<dyn FnOnce(From) -> To + 'm>>,
        }

        impl<'m, From, To> Dialog<To> for MappedDialog<'m, From, To> {
            fn show(&mut self, ctx: &egui::Context, dctx: &DialogContext) -> Option<To> {
                self.dialog.show(ctx, dctx).and_then(|from| {
                    self.mapper.take().map(|mapper| (mapper)(from))
                })
            }
        }

        DialogDetails {
            dialog: Box::new(MappedDialog {
                dialog: self.dialog,
                mapper: Some(handler),
            }),
            mask: self.mask,
            id: self.id,
        }
    }

    #[inline]
    /// Set whether to show a mask over the background.
    /// The mask will intercept all user interactions with the background.
    pub fn with_mask(mut self, mask: Option<Color32>) -> Self {
        self.mask = mask;
        self
    }

    #[inline]
    /// Check if a mask is set and return it if there is.
    pub fn mask(&self) -> Option<Color32> {
        self.mask
    }

    #[inline]
    /// Set the id of the dialog. Used for identify different dialogs
    /// with a AbstractDialog trait object.
    pub fn with_id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[inline]
    /// Check if an id is set and return it if there is.
    pub fn id(&self) -> Option<Id> {
        self.id
    }

    #[inline]
    /// Show the dialog.
    pub fn show(self, dialogs: &mut Dialogs<'a>) {
        dialogs.add(self);
    }

    #[inline]
    /// Show thre dialog if it is not already open.
    pub fn show_if_absent(self, dialogs: &mut Dialogs<'a>) {
        dialogs.add_if_absent(self);
    }
}

/// Alias for `DialogDetails<StandardReply>`
pub type StandardDialogDetails<'a> = DialogDetails<'a, StandardReply>;

impl StandardDialogDetails<'_> {
    #[inline]
    /// Create a `DialogDetails` struct with an info dialog.
    pub fn info(title: impl Into<WidgetText>, message: impl Into<WidgetText>) -> Self {
        StandardDialogDetails::new(
            StandardDialog::info(title, message)
        )
    }

    #[inline]
    /// Create a `DialogDetails` struct with a success dialog.
    pub fn success(title: impl Into<WidgetText>, message: impl Into<WidgetText>) -> Self {
        StandardDialogDetails::new(
            StandardDialog::success(title, message)
        )
    }

    #[inline]
    /// Create a `DialogDetails` struct with a confirm dialog.
    pub fn confirm(title: impl Into<WidgetText>, message: impl Into<WidgetText>) -> Self {
        StandardDialogDetails::new(
            StandardDialog::confirm(title, message)
        )
    }

    #[inline]
    /// Create a `DialogDetails` struct with a warning dialog.
    pub fn warning(title: impl Into<WidgetText>, message: impl Into<WidgetText>) -> Self {
        StandardDialogDetails::new(
            StandardDialog::warning(title, message)
        )
    }

    #[inline]
    /// Create a `DialogDetails` struct with an error dialog.
    pub fn error(title: impl Into<WidgetText>, message: impl Into<WidgetText>) -> Self {
        StandardDialogDetails::new(
            StandardDialog::error(title, message)
        )
    }
}

impl<'a> StandardDialogDetails<'a> {
    #[inline]
    /// Invoke handler when the dialog is accepted.
    pub fn on_accepted(self, handler: impl FnOnce() + 'a) -> Self {
        self.on_reply(|reply| {
            if reply.accepted() {
                (handler)();
            }
            reply
        })
    }

    #[inline]
    /// Invoke handler when the dialog is rejected.
    pub fn on_rejected(self, handler: impl FnOnce() + 'a) -> Self {
        self.on_reply(|reply| {
            if reply.rejected() {
                (handler)();
            }
            reply
        })
    }
    
    #[inline]
    /// Map to a DialogDetails with a new reply type using the handler
    /// which receives a boolean indicating whether the dialog was accepted.
    pub fn on_accepted_or<R: Any>(self, handler: impl FnOnce(bool) -> R + 'a) -> DialogDetails<'a, R> {
        self.on_reply(|reply| {
            (handler)(reply.accepted())
        })
    }

    #[inline]
    /// Map to a DialogDetails with boolean reply type
    /// indicating whether the dialog was accepted.
    pub fn map_accepted(self) -> DialogDetails<'a, bool> {
        self.on_reply(StandardReply::accepted)
    }

    #[inline]
    /// Map to a DialogDetails with boolean reply type
    /// indicating whether the dialog was rejected.
    pub fn map_rejected(self) -> DialogDetails<'a, bool> {
        self.on_reply(StandardReply::rejected)
    }

    #[inline]
    /// Map to a DialogDetails with a new reply type
    /// by specifying the accepted and rejected replies.
    pub fn map_accepted_or<R: Any>(self, accepted: R, rejected: R) -> DialogDetails<'a, R> {
        self.on_reply(|reply| {
            if reply.accepted() {
                accepted
            } else {
                rejected
            }
        })
    }
}
