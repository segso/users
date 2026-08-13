use iced::{
    Element,
    Length::Fill,
    border::rounded,
    widget::{column, container, horizontal_space, keyed_column, row, scrollable, text},
};
use user_registry_lib::User;

use crate::{
    constants::{
        BIG_TEXT_SIZE, BOLD_FONT, BUTTON_HEIGHT, BUTTON_PADDING, LARGE_TEXT_SIZE, MEDIUM_TEXT_SIZE,
        NORMAL_PADDING, NORMAL_SPACING, NORMAL_TEXT_SIZE, OUTER_BORDER_RADIUS, SMALL_PADDING,
        WINDOW_PADDING,
    },
    message::Message,
    state::State,
    utils::{create_danger_button, create_primary_button},
};

pub fn title() -> String {
    String::from("Users")
}

pub fn view(state: &State) -> Element<Message> {
    let mut users = keyed_column![].spacing(NORMAL_SPACING);

    for (id, user) in state.data.users() {
        users = users.push(id, create_user(id, user));
    }

    column![
        row![
            text("Users").size(LARGE_TEXT_SIZE).width(Fill),
            container(
                create_primary_button(text("Add").size(BIG_TEXT_SIZE).center())
                    .padding(BUTTON_PADDING)
                    .on_press(Message::Null)
            )
            .center_y(BUTTON_HEIGHT)
        ],
        scrollable(users).spacing(NORMAL_SPACING)
    ]
    .spacing(NORMAL_SPACING)
    .padding(WINDOW_PADDING)
    .into()
}

pub fn create_user(id: usize, user: &User) -> Element<Message> {
    container(row![
        horizontal_space().width(SMALL_PADDING),
        column![
            text(format!("{} {}", user.first_name, user.last_name))
                .size(MEDIUM_TEXT_SIZE)
                .font(BOLD_FONT),
            text(format!("ID: {id}"))
        ]
        .width(Fill),
        container(row![
            create_primary_button(text("Show").size(NORMAL_TEXT_SIZE).center())
                .on_press(Message::Null),
            horizontal_space().width(NORMAL_SPACING),
            create_danger_button(text("Remove").size(NORMAL_TEXT_SIZE).center())
                .on_press(Message::Null),
        ])
        .center_y(BUTTON_HEIGHT)
    ])
    .style(|theme| {
        container::Style::default()
            .background(theme.extended_palette().background.weak.color)
            .border(rounded(OUTER_BORDER_RADIUS))
    })
    .padding(NORMAL_PADDING)
    .width(Fill)
    .into()
}
