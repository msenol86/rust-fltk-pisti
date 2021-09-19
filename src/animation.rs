use fltk::{button::Button, prelude::*};
use fltk_theme::widget_themes;

pub fn set_button_color(but: &Button) {
    but.to_owned().set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    but.to_owned().set_label_size(24);
}
