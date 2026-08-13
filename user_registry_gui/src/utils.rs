use iced::{
    Element,
    border::rounded,
    widget::{Button, button},
};

use crate::constants::{BUTTON_HEIGHT, BUTTON_WIDTH, INNER_BORDER_RADIUS};

pub fn create_button<'a, T>(content: impl Into<Element<'a, T>>) -> Button<'a, T> {
    button(content).width(BUTTON_WIDTH).height(BUTTON_HEIGHT)
}

pub fn create_primary_button<'a, T>(content: impl Into<Element<'a, T>>) -> Button<'a, T> {
    create_button(content).style(|theme, status| button::Style {
        border: rounded(INNER_BORDER_RADIUS),
        ..button::primary(theme, status)
    })
}

pub fn create_danger_button<'a, T>(content: impl Into<Element<'a, T>>) -> Button<'a, T> {
    create_button(content).style(|theme, status| button::Style {
        border: rounded(INNER_BORDER_RADIUS),
        ..button::danger(theme, status)
    })
}
