use egui::{include_image, vec2, Align, Align2, Image, ImageSource, Label, Layout, Rect, Sense};
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
        (reply.localize(), reply)
    }
}

impl ToString for StandardReply {
    fn to_string(&self) -> String {
        self.localize()
    }
}

/// A standard dialog button with text and a reply
pub type StandardButton<Reply> = (String, Reply);

/// A standard dialog.
/// Use `Dialogs::info`, `Dialogs::warn`, ...
/// to directly show a standard dialog.
/// 
/// Or use `StandardDialog::info`, `StandardDialog::warn`, ...
/// to create a customizable standard dialog
pub struct StandardDialog<'i, Reply> {
    pub title: String,
    pub content: String,
    pub image: Option<ImageSource<'i>>,
    pub buttons: Vec<StandardButton<Reply>>,
}

/// Customize a standard dialog
impl<'i, Reply> StandardDialog<'i, Reply>
where Reply: Clone {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            content,
            image: None,
            buttons: vec![],
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    pub fn image(mut self, image: ImageSource<'i>) -> Self {
        self.image = Some(image);
        self
    }

    pub fn buttons(mut self, buttons: Vec<StandardButton<Reply>>) -> Self {
        self.buttons = buttons;
        self
    }
}

/// Build a standard dialog
impl<'i> StandardDialog<'i, StandardReply> {
    pub fn info(title: String, content: String) -> Self {
        Self {
            title,
            content,
            image: Some(ICON_INFO),
            buttons: vec![StandardReply::Ok.into()],
        }
    }

    pub fn success(title: String, content: String) -> Self {
        Self {
            title,
            content,
            image: Some(ICON_SUCCESS),
            buttons: vec![StandardReply::Ok.into()],
        }
    }

    pub fn confirm(title: String, content: String) -> Self {
        Self {
            title,
            content,
            image: Some(ICON_CONFIRM),
            buttons: vec![StandardReply::Yes.into(), StandardReply::No.into()],
        }
    }

    pub fn warning(title: String, content: String) -> Self {
        Self {
            title,
            content,
            image: Some(ICON_WARNING),
            buttons: vec![StandardReply::Ok.into()],
        }
    }

    pub fn error(title: String, content: String) -> Self {
        Self {
            title,
            content,
            image: Some(ICON_ERROR),
            buttons: vec![StandardReply::Ok.into()],
        }
    }
}

impl<'i, Reply> Dialog<Reply> for StandardDialog<'i, Reply>
where Reply: Clone {
    fn show(&mut self, ctx: &egui::Context, update_info: &DialogUpdateInfo) -> Option<Reply> {
        let Self {
            title,
            content,
            image,
            buttons,
        } = self;

        let mut reply = None;
        let mut open = !update_info.already_closed;

        let frame = egui::Frame::window(&ctx.style())
            .inner_margin(16.);

        egui::Window::new(title.as_str())
            .collapsible(false)
            .resizable(false)
            .pivot(Align2::CENTER_CENTER)
            .frame(frame)
            .id(update_info.dialog_id.unwrap_or("StandardDialog".into()))
            .constrain_to(update_info.mask_rect)
            .fade_in(update_info.animation.is_some())
            .fade_out(update_info.animation.is_some())
            .open(&mut open)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = vec2(8., 8.);
                ui.style_mut().override_font_id = Some(egui::FontId::proportional(15.));

                let text_rect = {
                    let mut size = ui.available_size();
                    size.x -= 40.;
                    size.y -= 80.;
                    Rect::from_min_size(ui.next_widget_position(), size)
                };
                
                let mut text_ui = ui.child_ui(
                    text_rect,
                    Layout::left_to_right(Align::TOP),
                    None
                );
                
                text_ui.horizontal(|ui| {
                    if let Some(image) = image {
                        ui.add(
                            Image::new(image.clone())
                                .fit_to_exact_size(vec2(48., 48.))
                        );
                    }
                    
                    ui.add(
                        Label::new(content.as_str())
                            .wrap()
                    );
                });

                ui.allocate_rect(text_ui.min_rect(), Sense::hover());
                ui.add_space(8.);
                
                ui.spacing_mut().button_padding = vec2(12., 4.);
                ui.style_mut().override_font_id = Some(egui::FontId::monospace(20.));

                let button_rect = {
                    let mut rect = ui.min_rect();
                    rect.min.y = rect.max.y;
                    rect.max.y = ui.available_rect_before_wrap().max.y;
                    rect
                };

                let mut button_ui = ui.child_ui(
                    button_rect,
                    Layout::right_to_left(Align::TOP),
                    None
                );

                for (text, reply_value) in buttons.iter().rev() {
                    if button_ui.button(text).clicked() {
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
