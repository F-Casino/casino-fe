use bevy::{
    app::AppExit, prelude::{Changed, EventWriter, Query, With}, render::color::Color, ui::{BackgroundColor, Interaction}
};

use crate::main_menu::{
    components::{ConnectToWalletButton, ExitButton}, HOVER_BUTTON_COLOR, NORMAL_BUTTON_COLOR,
    PRESSED_BUTTON_COLOR,
};

pub fn interact_with_connect_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ConnectToWalletButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVER_BUTTON_COLOR.into(),
            Interaction::Pressed => *background_color = PRESSED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_exit_button(
    mut event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ExitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Hovered => *background_color = HOVER_BUTTON_COLOR.into(),
            Interaction::Pressed => {
                event_writer.send(AppExit);
                *background_color = Color::RED.into()
            }
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
