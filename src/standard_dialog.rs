use egui::{include_image, vec2, Align, Align2, FontId, Image, ImageSource, Label, Layout, ScrollArea, Vec2, WidgetText};
use sys_locale::get_locales;

use crate::*;

// standard dialog icons
const ICON_INFO: ImageSource = include_image!("assets/info.svg");
const ICON_SUCCESS: ImageSource = include_image!("assets/success.svg");
const ICON_CONFIRM: ImageSource = include_image!("assets/confirm.svg");
const ICON_WARNING: ImageSource = include_image!("assets/warning.svg");
const ICON_ERROR: ImageSource = include_image!("assets/error.svg");

// we offer the same language supports as those offered by rust-lang.org
type StandardReplyTranslation = [(&'static str, &'static str); 10];

const STANDARD_OK_REPLY: StandardReplyTranslation = [  
    ("en-US", "OK"),  
    ("zh-CN", "确定"),  
    ("zh-TW", "確定"),  
    ("es", "OK"),
    ("fr", "D'accord"),  
    ("it", "OK"),
    ("ja", "はい"),  
    ("pt-BR", "OK"),
    ("ru", "Хорошо"),  
    ("tr", "Tamam"),  
];

const STANDARD_CANCEL_REPLY: StandardReplyTranslation = [  
    ("en-US", "Cancel"),  
    ("zh-CN", "取消"),  
    ("zh-TW", "取消"),  
    ("es", "Cancelar"),  
    ("fr", "Annuler"),  
    ("it", "Annulla"),  
    ("ja", "キャンセル"),  
    ("pt-BR", "Cancelar"),  
    ("ru", "Отмена"),  
    ("tr", "İptal"),  
];

const STANDARD_YES_REPLY: StandardReplyTranslation = [  
    ("en-US", "Yes"),  
    ("zh-CN", "是"),  
    ("zh-TW", "是"),  
    ("es", "Sí"),  
    ("fr", "Oui"),  
    ("it", "Sì"),  
    ("ja", "はい"),  
    ("pt-BR", "Sim"),  
    ("ru", "Да"),  
    ("tr", "Evet"),  
];

const STANDARD_NO_REPLY: [(&'static str, &'static str); 10] = [  
    ("en-US", "No"),  
    ("zh-CN", "否"),  
    ("zh-TW", "否"),  
    ("es", "No"),  
    ("fr", "Non"),  
    ("it", "No"),  
    ("ja", "いいえ"),  
    ("pt-BR", "Não"),  
    ("ru", "Нет"),  
    ("tr", "Hayır"),  
];

#[inline]
fn find_translation(source: StandardReplyTranslation, locale: String) -> Option<String> {
    for (locale_key, reply) in source {
        if locale_key == locale {
            return Some(reply.to_string());
        }
    }

    None
}

#[inline]
fn translate_standard_reply(source: StandardReplyTranslation) -> String {
    let locales = get_locales();
    for locale in locales {
        if let Some(reply) = find_translation(source, locale) {
            return reply;
        }
    }

    source[0].1.to_string()
}

/// Standard dialog replies.
/// Can be translated to the current locale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StandardReply {
    Ok,
    Cancel,
    Yes,
    No,
}

impl StandardReply {
    pub fn localize(self) -> String {
        match self {
            StandardReply::Ok =>     translate_standard_reply(STANDARD_OK_REPLY),
            StandardReply::Cancel => translate_standard_reply(STANDARD_CANCEL_REPLY),
            StandardReply::Yes =>    translate_standard_reply(STANDARD_YES_REPLY),
            StandardReply::No =>     translate_standard_reply(STANDARD_NO_REPLY),
        }
    }

    #[inline]
    pub fn accepted(self) -> bool {
        match self {
            StandardReply::Ok | StandardReply::Yes => true,
            _ => false,
        }
    }

    #[inline]
    pub fn rejected(self) -> bool {
        match self {
            StandardReply::Cancel | StandardReply::No => true,
            _ => false,
        }
    }
}

impl From<StandardReply> for StandardButton<StandardReply> {
    fn from(reply: StandardReply) -> Self {
        (reply.localize().into(), reply)
    }
}

impl ToString for StandardReply {
    fn to_string(&self) -> String {
        self.localize()
    }
}

/// A standard dialog button with text and a reply
pub type StandardButton<Reply> = (WidgetText, Reply);

/// A standard dialog.
/// Use `Dialogs::info`, `Dialogs::warn`, ...
/// to directly show a standard dialog.
/// 
/// Or use `StandardDialog::info`, `StandardDialog::warn`, ...
/// to create a customizable standard dialog
pub struct StandardDialog<'i, Reply> {
    pub title: WidgetText,
    pub content: WidgetText,
    pub image: Option<ImageSource<'i>>,
    pub buttons: Vec<StandardButton<Reply>>,
    pub min_size: Vec2,
    pub max_size: Vec2,
}

/// Customize a standard dialog
impl<'i, Reply> StandardDialog<'i, Reply>
where Reply: Clone {
    pub fn new(title: impl Into<WidgetText>, content: impl Into<WidgetText>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            image: None,
            buttons: vec![],
            min_size: Vec2::ZERO,
            max_size: Vec2::INFINITY,
        }
    }
    
    /// Set the dialog title
    #[inline]
    pub fn title(mut self, title: impl Into<WidgetText>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the dialog content
    #[inline]
    pub fn content(mut self, content: impl Into<WidgetText>) -> Self {
        self.content = content.into();
        self
    }

    /// Set the dialog image
    #[inline]
    pub fn image(mut self, image: ImageSource<'i>) -> Self {
        self.image = Some(image);
        self
    }

    /// Set the dialog buttons
    #[inline]
    pub fn buttons(mut self, buttons: Vec<StandardButton<Reply>>) -> Self {
        self.buttons = buttons;
        self
    }

    /// Add a button to the dialog
    #[inline]
    pub fn push_button(mut self, button: StandardButton<Reply>) -> Self {
        self.buttons.push(button);
        self
    }

    /// Set the minimum size of the dialog
    #[inline]
    pub fn min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = min_size;
        self
    }

    /// Set the maximum size of the dialog
    #[inline]
    pub fn max_size(mut self, max_size: Vec2) -> Self {
        self.max_size = max_size;
        self
    }
}

/// Build a standard dialog
impl<'i> StandardDialog<'i, StandardReply> {
    /// Create an info dialog
    pub fn info(title: impl Into<WidgetText>, content: impl Into<WidgetText>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            image: Some(ICON_INFO),
            buttons: vec![StandardReply::Ok.into()],
            min_size: Vec2::ZERO,
            max_size: Vec2::INFINITY,
        }
    }

    /// Create a success dialog
    pub fn success(title: impl Into<WidgetText>, content: impl Into<WidgetText>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            image: Some(ICON_SUCCESS),
            buttons: vec![StandardReply::Ok.into()],
            min_size: Vec2::ZERO,
            max_size: Vec2::INFINITY,
        }
    }

    /// Create a confirmation dialog
    pub fn confirm(title: impl Into<WidgetText>, content: impl Into<WidgetText>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            image: Some(ICON_CONFIRM),
            buttons: vec![StandardReply::Yes.into(), StandardReply::No.into()],
            min_size: Vec2::ZERO,
            max_size: Vec2::INFINITY,
        }
    }

    /// Create a warning dialog
    pub fn warning(title: impl Into<WidgetText>, content: impl Into<WidgetText>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            image: Some(ICON_WARNING),
            buttons: vec![StandardReply::Ok.into()],
            min_size: Vec2::ZERO,
            max_size: Vec2::INFINITY,
        }
    }

    /// Create an error dialog
    pub fn error(title: impl Into<WidgetText>, content: impl Into<WidgetText>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            image: Some(ICON_ERROR),
            buttons: vec![StandardReply::Ok.into()],
            min_size: Vec2::ZERO,
            max_size: Vec2::INFINITY,
        }
    }
}

impl<'i, Reply> Dialog<Reply> for StandardDialog<'i, Reply>
where Reply: Clone {
    fn show(&mut self, ctx: &egui::Context, dctx: &DialogContext) -> Option<Reply> {
        let Self {
            title,
            content,
            image,
            buttons,
            min_size,
            max_size,
        } = self;

        let mut reply = None;
        let mut open = true;

        closable_dialog_window(ctx, dctx, title.clone(), &mut open)
            // explicitly center the dialog as our button layout depends on this
            .anchor(Align2::CENTER_CENTER, [0., 0.])
            .min_size(min_size.max(dctx.min_size.unwrap_or(Vec2::ZERO)))
            .max_size(max_size.min(dctx.max_size.unwrap_or(Vec2::INFINITY)))
            .show(ctx, |ui| {
                ui.style_mut().override_font_id = Some(FontId::proportional(16.0));
                
                ui.horizontal_top(|ui| {
                    const IMAGE_WIDTH: f32 = 48.;
                    
                    let max_width = dctx.mask_rect.width()
                        - ui.spacing().window_margin.right
                        - 24.; // reserve some space for better appearance
                    let max_height = {
                        let text_height = ui.style()
                            .text_styles
                            .get(&egui::TextStyle::Button)
                            .map(|f| f.size)
                            .unwrap_or(20.)
                            * 1.5;

                        dctx.mask_rect.bottom()
                            - ui.next_widget_position().y
                            - ui.spacing().item_spacing.y
                            - ui.spacing().button_padding.y * 2.
                            - text_height
                            - ui.spacing().window_margin.bottom
                            - 24. // reserve some space for better appearance
                    };

                    ui.set_max_size(vec2(max_width, max_height));

                    if let Some(image) = image {
                        ui.add(
                            Image::new(image.clone())
                                .fit_to_exact_size(vec2(IMAGE_WIDTH, IMAGE_WIDTH))
                        );
                    }
                    
                    ScrollArea::vertical()
                        .auto_shrink([true, true])
                        .show(ui, |ui| {
                            ui.add(
                                Label::new(content.clone())
                                    .wrap()
                            );
                        });
                });

                let layout = if ui.is_sizing_pass() {
                    Layout::left_to_right(Align::Min)
                } else {
                    // calc the available width for button by making sure that the dialog is centered
                    ui.set_max_width((ctx.screen_rect().center().x - ui.next_widget_position().x).abs() * 2.);
                    Layout::right_to_left(Align::Min)
                };
                ui.with_layout(layout, |ui| {
                    for (text, reply_value) in buttons.iter().rev() {
                        if ui.button(text.clone()).clicked() {
                            reply = Some(reply_value.clone());
                            break;
                        }
                    }
                });
            });

        if let Some(reply_value) = reply {
            Some(reply_value.clone())
        } else if !open {
            buttons.last().map(|(_, reply_value)| reply_value.clone())
        } else {
            None
        }
    }
}

/// Create a suggested dialog window
pub fn dialog_window<'open>(
    ctx: &egui::Context,
    dctx: &DialogContext,
    title: impl Into<WidgetText>
) -> egui::Window<'open> {
    let frame = egui::Frame::window(&ctx.style())
        .inner_margin(16.);

    let mut window = egui::Window::new(title.into())
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .frame(frame)
        .fade_in(dctx.animation.is_some())
        .fade_out(dctx.animation.is_some())
        .interactable(!dctx.already_closed);

    if let Some(min_size) = dctx.min_size {
        window = window.min_size(min_size);
    }

    if let Some(max_size) = dctx.max_size {
        window = window.max_size(max_size);
    }

    if let Some(id) = dctx.dialog_id {
        window = window.id(id);
    }
    
    window
}

/// Create a suggested dialog window with a close button
#[inline]
pub fn closable_dialog_window<'open>(
    ctx: &egui::Context,
    dctx: &DialogContext,
    title: impl Into<WidgetText>,
    open: &'open mut bool
) -> egui::Window<'open> {
    if dctx.already_closed {
        *open = false;
    }

    dialog_window(ctx, dctx, title).open(open)
}
