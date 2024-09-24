use egui::{include_image, vec2, Align, Align2, Image, ImageSource, Label, Layout, Rect, Sense, WidgetText};
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
    pub fn localize(&self) -> String {
        match *self {
            StandardReply::Ok =>     translate_standard_reply(STANDARD_OK_REPLY),
            StandardReply::Cancel => translate_standard_reply(STANDARD_CANCEL_REPLY),
            StandardReply::Yes =>    translate_standard_reply(STANDARD_YES_REPLY),
            StandardReply::No =>     translate_standard_reply(STANDARD_NO_REPLY),
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
        }
    }
    
    /// Set the dialog title
    pub fn title(mut self, title: impl Into<WidgetText>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the dialog content
    pub fn content(mut self, content: impl Into<WidgetText>) -> Self {
        self.content = content.into();
        self
    }

    /// Set the dialog image
    pub fn image(mut self, image: ImageSource<'i>) -> Self {
        self.image = Some(image);
        self
    }

    /// Set the dialog buttons
    pub fn buttons(mut self, buttons: Vec<StandardButton<Reply>>) -> Self {
        self.buttons = buttons;
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
        }
    }

    /// Create a success dialog
    pub fn success(title: impl Into<WidgetText>, content: impl Into<WidgetText>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            image: Some(ICON_SUCCESS),
            buttons: vec![StandardReply::Ok.into()],
        }
    }

    /// Create a confirmation dialog
    pub fn confirm(title: impl Into<WidgetText>, content: impl Into<WidgetText>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            image: Some(ICON_CONFIRM),
            buttons: vec![StandardReply::Yes.into(), StandardReply::No.into()],
        }
    }

    /// Create a warning dialog
    pub fn warning(title: impl Into<WidgetText>, content: impl Into<WidgetText>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            image: Some(ICON_WARNING),
            buttons: vec![StandardReply::Ok.into()],
        }
    }

    /// Create an error dialog
    pub fn error(title: impl Into<WidgetText>, content: impl Into<WidgetText>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            image: Some(ICON_ERROR),
            buttons: vec![StandardReply::Ok.into()],
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
        } = self;

        let mut reply = None;
        let mut open = true;

        closable_dialog_window(ctx, dctx, title.clone(), &mut open)
            .show(ctx, |ui| {
                let available_rect = dctx.mask_rect - ctx.style().spacing.window_margin;
                let content_rect = {
                    let mut size = available_rect.size();
                    size.x = size.x.min(800.);
                    Rect::from_min_size(ui.next_widget_position(), size)
                };

                let mut content_ui = ui.child_ui(
                    content_rect,
                    *ui.layout(),
                    None
                );
                
                content_ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = vec2(8., 8.);
                    ui.style_mut().override_font_id = Some(egui::FontId::proportional(15.));

                    if let Some(image) = image {
                        ui.add(
                            Image::new(image.clone())
                                .fit_to_exact_size(vec2(48., 48.))
                        );
                    }
                    
                    ui.add(
                        Label::new(content.clone())
                            .wrap()
                    );
                });

                ui.allocate_rect(content_ui.min_rect(), Sense::hover());
                
                ui.add_space(8.);

                let button_rect = {
                    let mut rect = ui.min_rect();
                    rect.min.x = ctx.screen_rect().min.x;
                    rect.min.y = rect.max.y;
                    rect.max.y = ui.available_rect_before_wrap().max.y;
                    rect
                };

                let mut button_ui = ui.child_ui(
                    button_rect,
                    Layout::right_to_left(Align::TOP),
                    None
                );
                
                button_ui.spacing_mut().button_padding = vec2(12., 4.);
                button_ui.style_mut().override_font_id = Some(egui::FontId::monospace(20.));

                for (text, reply_value) in buttons.iter().rev() {
                    if button_ui.button(text.clone()).clicked() {
                        reply = Some(reply_value.clone());
                        break;
                    }
                }

                ui.allocate_rect(button_ui.min_rect(), Sense::hover());
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

    let window = egui::Window::new(title.into())
        .collapsible(false)
        .resizable(false)
        .pivot(Align2::CENTER_CENTER)
        .frame(frame)
        .constrain_to(dctx.mask_rect)
        .fade_in(dctx.animation.is_some())
        .fade_out(dctx.animation.is_some())
        .interactable(!dctx.already_closed);

    if let Some(id) = dctx.dialog_id {
        window.id(id)
    } else {
        window
    }
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
