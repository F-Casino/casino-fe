use bevy::{
    asset::AssetServer,
    hierarchy::BuildChildren,
    prelude::{Commands, Entity, Res},
    render::color::Color,
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle}, AlignItems, Display, FlexWrap, JustifyContent, Style, UiImage, Val
    },
};

use crate::in_game::components::{
    Background, Container, ContainerGame, ContainerInformationUser, Dice, Plate,
    SettingButton, Table,
};

pub fn spawn_in_game (mut commands: Commands, asset_server: Res<AssetServer>) {
    build_in_game(commands, asset_server);
}

fn build_in_game(mut commands: Commands, asset_server: Res<AssetServer>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Background {},
        ))
        .with_children(|parent| {
            // === Container Information User ===
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(30.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ContainerInformationUser {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        SettingButton {},
                    ));
                });
            // === End Container Information User===

            // === Container Game ===
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(70.0),
                            height: Val::Percent(100.0),
                            flex_wrap: FlexWrap::Wrap,
                            ..Default::default()
                        },
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    },
                    ContainerGame {},
                ))
                .with_children(|parent| {
                    // === Table ===
                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(70.0),
                                ..Default::default()
                            },
                            image: UiImage::new(asset_server.load("baucuatomca_game_mat.png")),
                            ..Default::default()
                        },
                        Table {},
                    ));
                    // === End Table ===

                    // === Plate ===
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(30.0),
                                    justify_content: JustifyContent::SpaceAround,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            Plate {},
                        ))
                        .with_children(|parent| {
                            // === Dice ===
                            parent.spawn((
                                ImageBundle {
                                    style: Style {
                                        width: Val::Percent(30.0),
                                        height: Val::Percent(70.0),
                                        ..Default::default()
                                    },
                                    background_color: Color::RED.into(),
                                    ..Default::default()
                                },
                                Dice {},
                            ));
                            parent.spawn((
                                ImageBundle {
                                    style: Style {
                                        width: Val::Percent(30.0),
                                        height: Val::Percent(70.0),
                                        ..Default::default()
                                    },
                                    background_color: Color::BLACK.into(),
                                    ..Default::default()
                                },
                                Dice {},
                            ));
                            parent.spawn((
                                ImageBundle {
                                    style: Style {
                                        width: Val::Percent(30.0),
                                        height: Val::Percent(70.0),
                                        ..Default::default()
                                    },
                                    background_color: Color::BLUE.into(),
                                    ..Default::default()
                                },
                                Dice {},
                            ));
                            // === End Dice ===
                        });
                    // === End Plate ===
                });
            // === End Container Game ===
        })
        .id()
}

