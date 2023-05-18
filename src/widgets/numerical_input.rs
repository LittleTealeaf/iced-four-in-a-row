use iced::{
    widget::{button, row, text},
    Element,
};

pub fn numerical_input<'a, Message: Clone + 'a>(
    value: isize,
    on_decrement: Message,
    on_increment: Message,
) -> Element<'a, Message, iced::Renderer> {
    row(vec![
        button("-").on_press(on_decrement).into(),
        text(value).into(),
        button("+").on_press(on_increment).into(),
    ])
    .align_items(iced::Alignment::Center)
    .spacing(3)
    .into()
}
