mod markers;
mod picker;
mod utils;
mod value;

use self::{markers::*, picker::*, value::*};
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
        .add_plugins((ValuePlugin, PickerPlugin))
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
            (update_ui_text, update_ui_bg).in_set(UpdateMarker::Ui),
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
                        crate::markers::HexTextMarker,
                    ));
                });
        });
}

fn update_ui_text(
    color_value: Res<ColorValue>,
    hex_value: Res<HexValue>,
    mut text: Single<(&mut Text, &mut TextColor), With<HexTextMarker>>,
) {
    if !hex_value.is_changed() {
        return;
    }

    text.0.0 = hex_value.0.clone();
    text.1.0 = if color_value.0.luminance() > 0.5 {
        Color::BLACK
    } else {
        Color::WHITE
    };
}

fn update_ui_bg(color_value: Res<ColorValue>, mut clear_color: ResMut<ClearColor>) {
    if color_value.is_changed() {
        clear_color.0 = color_value.0;
    }
}
