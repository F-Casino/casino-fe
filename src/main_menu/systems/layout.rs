use bevy::{
    asset::AssetServer,
    hierarchy::BuildChildren,
    prelude::{Commands, Entity, Query, Res, With},
    text::TextStyle,
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle},
        AlignItems, FlexDirection, JustifyContent, PositionType, Style, UiImage, Val,
    },
};

use crate::main_menu::{
    components::{Background, ConnectToWalletButton, ExitButton, Logo, MainMenu},
    BUTTON_STYLE, NORMAL_BUTTON_COLOR,
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, menu_entity: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = menu_entity.get_single() {
        commands.entity(main_menu_entity).despawn();
    }
}

fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Percent(5.0),
                    column_gap: Val::Percent(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // === Logo ===
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Percent(25.0),
                        height: Val::Percent(10.0),
                        ..Default::default()
                    },
                    image: UiImage::new(asset_server.load("logo.png")),
                    ..Default::default()
                },
                Logo {},
            ));

            // === Button ===
            // == Connect to wallet ==
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..Default::default()
                    },
                    ConnectToWalletButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Connect to wallet",
                        TextStyle {
                            font_size: 30.,
                            ..Default::default()
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..Default::default()
                    },
                    ExitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "EXIT",
                        TextStyle {
                            font_size: 30.,
                            ..Default::default()
                        },
                    ));
                });
        })
        .id()
}
