use std::collections::VecDeque;

use egui::{Color32, Id, LayerId, Margin, Rect, Rounding, Sense, Ui, UiStackInfo};

use crate::*;

/// Information about the current dialog update.
pub struct DialogContext {
    /// The updated dialog id if there is one.
    pub dialog_id: Option<Id>,

    /// The current animation function.
    /// If None, the dialog will not be animated.
    pub animation: Option<fn(f32) -> f32>,

    /// The current opacity of the dialog.
    /// Used for animation.
    pub opacity: f32,

    /// Whether the dialog has been closed.
    /// If animation is enabled, this will be true if the dialog is fading out.
    pub already_closed: bool,

    /// The dialog's mask rect.
    pub mask_rect: Rect,
}

/// Abstraction over dialog details with different reply types.
pub trait AbstractDialog {
    /// Paint the current frame and return whether the dialog has been replied.
    fn update(&mut self, ctx: &egui::Context, dctx: &DialogContext) -> bool;

    /// Return the mask color if there is one.
    fn mask(&self) -> Option<Color32>;

    fn id(&self) -> Option<Id>;
}

impl<'a, Reply> AbstractDialog for DialogDetails<'a, Reply> {
    fn update(&mut self, ctx: &egui::Context, dctx: &DialogContext) -> bool {
        match self.dialog.show(ctx, dctx) {
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
    
    /// The margin of the mask.
    /// This is useful if your window has a transparent margin or shadow.
    pub mask_margin: Margin,
    /// The rounding of the mask.
    /// This is useful if your window has rounded corners.
    pub mask_rounding: Rounding,
    
    /// The animation function.
    /// Set to None to disable animation.
    pub animation: Option<fn(f32) -> f32>,
    
    fading_dialog: Option<Box<dyn AbstractDialog + 'a>>,
}

impl Dialogs<'_> {
    #[inline]
    pub fn new() -> Self {
        Self {
            dialogs: VecDeque::new(),
            mask_margin: Margin::ZERO,
            mask_rounding: Rounding::ZERO,
            animation: Some(egui::emath::easing::cubic_out),
            fading_dialog: None,
        }
    }

    #[inline]
    pub fn mask_margin(mut self, margin: impl Into<Margin>) -> Self {
        self.mask_margin = margin.into();
        self
    }

    #[inline]
    pub fn mask_rounding(mut self, rounding: impl Into<Rounding>) -> Self {
        self.mask_rounding = rounding.into();
        self
    }

    pub fn animate(mut self, animation: Option<fn(f32) -> f32>) -> Self {
        self.animation = animation;
        if animation.is_none() && self.fading_dialog.is_some() {
            self.fading_dialog = None;
        }
        self
    }

    pub fn animated(mut self, is_animated: bool) -> Self {
        if is_animated {
            if self.animation.is_none() {
                self.animation = Some(egui::emath::easing::cubic_out);
            }
        } else {
            self.animation = None;
            self.fading_dialog = None;
        }
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
    const ID_NAME: &'static str = "dialog_mask";
    
    /// Paint a mask with the given color.
    /// This will intercept all user interactions with background.
    /// Returns the painted opacity.
    pub fn show_mask(&self, ctx: &egui::Context, color: Color32, dialog_on: bool) -> f32 {
        let id = Id::new((ctx.viewport_id(), Self::ID_NAME));
        
        let how_on = match self.animation {
            Some(easing) => {
                let value = ctx.animate_bool_with_easing(
                    id,
                    dialog_on,
                    easing
                );
                if value == 0. {
                    return 0.;
                }
                value
            },
            None => if dialog_on { 1. } else { return 0.; },
        };
        
        let layer_id = LayerId {
            order: egui::Order::PanelResizeLine,
            id,
        };

        let mask_rect = ctx.screen_rect() - self.mask_margin;
        let mut mask_ui = Ui::new(
            ctx.clone(),
            layer_id,
            id,
            mask_rect,
            mask_rect,
            UiStackInfo::default(), // set by show_inside_dyn
        );

        mask_ui.set_opacity(how_on);

        mask_ui.painter().rect_filled(mask_rect, self.mask_rounding, color);
        // sense all interactions to forbid interact with background widgets
        mask_ui.allocate_rect(mask_rect, Sense::click_and_drag());
        
        how_on
    }

    /// Show the currently open dialog if there is one.
    pub fn show(&mut self, ctx: &egui::Context) {
        let on = !self.dialogs.is_empty() && self.fading_dialog.is_none();
        let how_on = if on || self.fading_dialog.is_some() {
            let mask_color = match &self.fading_dialog {
                Some(fading_dialog) => fading_dialog.mask(),
                None => self.dialogs.front().unwrap().mask(), // self.dialogs mustn't be empty here
            };
            if let Some(mask_color) = mask_color {
                self.show_mask(ctx, mask_color, on)
            } else if let Some(animation) = self.animation {
                ctx.animate_bool_with_easing(
                    Id::new((ctx.viewport_id(), Self::ID_NAME)),
                    on,
                    animation
                )
            } else {
                1.
            }
        } else {
            0.
        };

        if how_on == 0. {
            if self.fading_dialog.is_some() {
                self.fading_dialog = None;
                ctx.request_repaint();
            }
            return;
        }

        let (dialog, already_closed) = match self.fading_dialog {
            Some(ref mut fading_dialog) => (Some(fading_dialog), true),
            None => (self.dialogs.front_mut(), false),
        };
        
        if let Some(dialog) = dialog {
            let dctx = DialogContext {
                dialog_id: dialog.id(),
                animation: self.animation,
                opacity: how_on,
                already_closed,
                mask_rect: ctx.screen_rect() - self.mask_margin,
            };
            if dialog.update(ctx, &dctx) && !already_closed {
                let closed_dialog = self.dialogs.pop_front();
                if self.animation.is_some() {
                    self.fading_dialog = closed_dialog;
                }
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
