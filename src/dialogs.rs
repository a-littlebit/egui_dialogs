use std::collections::VecDeque;

use egui::{Color32, Id, LayerId, Margin, Rounding, Sense, Ui, UiStackInfo};

use crate::*;

/// Abstraction over dialog details with different reply types.
pub trait AbstractDialog {
    /// Paint the current frame and return whether the dialog has been replied.
    fn update(&mut self, ctx: &egui::Context) -> bool;

    /// Return the mask color if there is one.
    fn mask(&self) -> Option<Color32>;

    fn id(&self) -> Option<Id>;
}

impl<'a, Reply> AbstractDialog for DialogDetails<'a, Reply> {
    fn update(&mut self, ctx: &egui::Context) -> bool {
        match self.dialog.show(ctx, self.id) {
            Some(reply) => {
                self.handler.take().map(|handler| handler(reply));
                true
            },
            None => false
        }
    }

    fn mask(&self) -> Option<Color32> {
        self.mask
    }

    fn id(&self) -> Option<Id> {
        self.id
    }
}

/// A dialog manager for showing dialogs on an egui::Context.
/// 
/// # Example
/// ```rust
/// use egui_dialogs::Dialogs;
/// 
/// struct MyApp<'a> {
///     dialogs: Dialogs<'a>,
/// }
/// 
/// impl<'a> MyApp<'_> {
///     fn new(_cc: &eframe::CreationContext<'_>) -> Self {
///         MyApp {
///             dialogs: Dialogs::new(),
///         }
///     }
/// }
/// 
/// impl eframe::App for MyApp<'_> {
///     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
///         self.dialogs.show(ctx);
/// 
///         // when you want to show a dialog
///         self.dialogs.info("Hello", "This is a message");
///     }
/// }
/// ```
pub struct Dialogs<'a> {
    dialogs: VecDeque<Box<dyn AbstractDialog + 'a>>,
    mask_margin: Margin,
    mask_rounding: Rounding
}

impl Dialogs<'_> {
    #[inline]
    pub fn new() -> Self {
        Self {
            dialogs: VecDeque::new(),
            mask_margin: Margin::ZERO,
            mask_rounding: Rounding::ZERO,
        }
    }

    #[inline]
    /// Change the margin of the mask.
    /// This is useful if your window has a transparent margin or shadow.
    pub fn mask_margin(mut self, margin: impl Into<Margin>) -> Self {
        self.mask_margin = margin.into();
        self
    }

    #[inline]
    /// Change the rounding of the mask.
    /// This is useful if your window has rounded corners.
    pub fn mask_rounding(mut self, rounding: impl Into<Rounding>) -> Self {
        self.mask_rounding = rounding.into();
        self
    }
}

impl Default for Dialogs<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Dialogs<'a> {
    /// Show a dialog.
    /// If a dialog is already open, the new dialog will be added to the back of the queue.
    #[inline]
    pub fn add<Reply: 'a>(&mut self, dialog: DialogDetails<'a, Reply>) {
        self.dialogs.push_back(Box::new(dialog));
    }

    /// Show a dialog immediately.
    /// This means it will cut into the front of the current dialog queue.
    #[inline]
    pub fn add_immediate<Reply: 'a>(&mut self, dialog: DialogDetails<'a, Reply>) {
        self.dialogs.push_front(Box::new(dialog));
    }

    /// Get the currently open dialog.
    #[inline]
    pub fn current_dialog(&self) -> Option<&Box<dyn AbstractDialog + 'a>> {
        self.dialogs.front()
    }

    /// Pop the current dialog.
    #[inline]
    pub fn pop_front(&mut self) -> Option<Box<dyn AbstractDialog + 'a>> {
        self.dialogs.pop_front()
    }

    /// Get the last dialog.
    #[inline]
    pub fn last_dialog(&self) -> Option<&Box<dyn AbstractDialog + 'a>> {
        self.dialogs.back()
    }

    /// Pop the last dialog.
    #[inline]
    pub fn pop_back(&mut self) -> Option<Box<dyn AbstractDialog + 'a>> {
        self.dialogs.pop_back()
    }

    /// Get the number of dialogs in the queue.
    #[inline]
    pub fn count(&self) -> usize {
        self.dialogs.len()
    }

    /// Get the dialog queue.
    #[inline]
    pub fn dialogs(&self) -> &VecDeque<Box<dyn AbstractDialog + 'a>> {
        &self.dialogs
    }

    /// Get the dialog queue mutably.
    #[inline]
    pub fn dialogs_mut(&mut self) -> &mut VecDeque<Box<dyn AbstractDialog + 'a>> {
        &mut self.dialogs
    }
}

impl Dialogs<'_> {
    /// Paint a mask with the given color.
    /// This will intercept all user interactions with background.
    pub fn show_mask(&self, ctx: &egui::Context, color: Color32) {
        let layer_id = LayerId {
            order: egui::Order::PanelResizeLine,
            id: Id::new("dialog_mask"),
        };
        let id = Id::new((ctx.viewport_id(), "dialog_mask"));

        let mask_rect = ctx.screen_rect() - self.mask_margin;
        let mut panel_ui = Ui::new(
            ctx.clone(),
            layer_id,
            id,
            mask_rect,
            mask_rect,
            UiStackInfo::default(), // set by show_inside_dyn
        );

        panel_ui.painter().rect_filled(mask_rect, self.mask_rounding, color);
        // sense all interactions to forbid interact with background widgets
        panel_ui.allocate_rect(mask_rect, Sense::click_and_drag());
    }

    /// Show the currently open dialog if there is one.
    pub fn show(&mut self, ctx: &egui::Context) {
        if let Some(dialog) = self.dialogs.front() {
            if let Some(mask) = dialog.mask() {
                self.show_mask(ctx, mask);
            }
        }
        if let Some(dialog) = self.dialogs.front_mut() {
            if dialog.update(ctx) {
                self.dialogs.pop_front();
            }
        }
    }
}

impl<'a> Dialogs<'a> {
    #[inline]
    /// Show an information dialog.
    pub fn info(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.add(StandardDialogDetails::info(title, message));
    }

    #[inline]
    /// Show a success dialog.
    pub fn success(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.add(StandardDialogDetails::success(title, message));
    }

    #[inline]
    /// Show a confirmation dialog and handle the reply.
    pub fn confirm(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        handler: impl FnOnce(StandardReply) + 'a
    ) {
        self.add(
            StandardDialogDetails::confirm(title, message)
                .on_reply(handler)
        );
    }

    #[inline]
    /// Show a warning dialog.
    pub fn warning(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.add(StandardDialogDetails::warning(title, message));
    }

    #[inline]
    /// Show an error dialog.
    pub fn error(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.add(StandardDialogDetails::error(title, message));
    }
}
