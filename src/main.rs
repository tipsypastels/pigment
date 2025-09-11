mod clipboard;
mod draggable;
mod markers;
mod menu;
mod picker;
mod value;

use self::{clipboard::*, draggable::*, markers::*, menu::*, picker::*, value::*};
use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{WindowLevel, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(375.0, 200.0),
                        window_level: WindowLevel::AlwaysOnTop,
                        decorations: false,
                        resizable: false,
                        title: "Pigment Color Picker".into(),
                        ..default()
                    }),
                    ..default()
                })
                .disable::<LogPlugin>(),
        )
        .add_plugins((
            ValuePlugin,
            PickerPlugin,
            ClipboardPlugin,
            MenuPlugin,
            DraggablePlugin,
        ))
        .insert_resource(ClearColor(Color::WHITE))
        .configure_sets(
            Update,
            (
                UpdateMarker::Logic.after(UpdateMarker::Interaction),
                UpdateMarker::Ui.after(UpdateMarker::Logic),
            ),
        )
        .add_systems(Startup, setup_ui)
        .add_systems(
            Update,
            (
                update_ui_text_hex,
                update_ui_text_color,
                update_ui_bg,
                update_ui_picker_control_text,
            )
                .in_set(UpdateMarker::Ui),
        )
        .run();
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(2.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderColor(Color::WHITE),
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(6.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new("#FFFFFFF"),
                        TextFont {
                            font: asset_server.load("fonts/Monoton.ttf"),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                        HexTextMarker,
                    ));

                    builder
                        .spawn(Node {
                            margin: UiRect::top(Val::Px(15.0)),
                            column_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn((
                                Text::new("Pick [Sp]"),
                                TextFont {
                                    font: asset_server.load("fonts/CourierPrime.ttf"),
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                                PickerControlTextMarker,
                            ));

                            builder.spawn((
                                Text::new("Copy [C]"),
                                TextFont {
                                    font: asset_server.load("fonts/CourierPrime.ttf"),
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                        });
                });
        });
}

fn update_ui_text_hex(hex_value: Res<HexValue>, mut text: Single<&mut Text, With<HexTextMarker>>) {
    if hex_value.is_changed() {
        text.0 = hex_value.0.clone();
    }
}

fn update_ui_text_color(color_value: Res<ColorValue>, text_query: Query<&mut TextColor>) {
    if color_value.is_changed() {
        for mut text in text_query {
            text.0 = if color_value.0.luminance() > 0.5 {
                Color::BLACK
            } else {
                Color::WHITE
            };
        }
    }
}

fn update_ui_bg(color_value: Res<ColorValue>, mut clear_color: ResMut<ClearColor>) {
    if color_value.is_changed() {
        clear_color.0 = color_value.0;
    }
}

fn update_ui_picker_control_text(
    asset_server: Res<AssetServer>,
    picker_open: Res<PickerOpen>,
    mut font: Single<&mut TextFont, With<PickerControlTextMarker>>,
) {
    if picker_open.is_changed() {
        font.font = if picker_open.0 {
            asset_server.load("fonts/CourierPrime-Bold.ttf")
        } else {
            asset_server.load("fonts/CourierPrime.ttf")
        }
    }
}
