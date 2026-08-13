use iced::{Font, font::Weight};

pub const WINDOW_PADDING: u16 = 15;

pub const NORMAL_PADDING: u16 = 10;
pub const SMALL_PADDING: u16 = 4;

pub const NORMAL_SPACING: u16 = 10;

pub const INNER_BORDER_RADIUS: u16 = 10;
pub const OUTER_BORDER_RADIUS: u16 = 22;

pub const LARGE_TEXT_SIZE: u16 = 30;
pub const BIG_TEXT_SIZE: u16 = 20;
pub const MEDIUM_TEXT_SIZE: u16 = 18;
pub const NORMAL_TEXT_SIZE: u16 = 15;

pub const BUTTON_PADDING_HORIZONTAL: u16 = 15;
pub const BUTTON_PADDING_VERTICAL: u16 = 5;
pub const BUTTON_PADDING: [u16; 2] = [BUTTON_PADDING_VERTICAL, BUTTON_PADDING_HORIZONTAL];

pub const BUTTON_WIDTH: u16 = 80;
pub const BUTTON_HEIGHT: u16 = 44;

pub const BOLD_FONT: Font = {
    let mut font = Font::DEFAULT;
    font.weight = Weight::Bold;
    font
};
