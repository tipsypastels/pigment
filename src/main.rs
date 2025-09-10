mod clipboard;
mod markers;
mod picker;

use bevy::{
    input::common_conditions::input_just_pressed,
    log::LogPlugin,
    prelude::*,
    window::{WindowLevel, WindowResolution},
};
use block2::RcBlock;
use objc2_app_kit::{NSColor, NSColorSampler};
use std::sync::{OnceLock, mpsc};

static PICK_TX: OnceLock<mpsc::Sender<Color>> = OnceLock::new();

fn main() {
    let (tx, rx) = mpsc::channel();
    let _ = PICK_TX.set(tx);

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
        .insert_resource(ClearColor(Color::WHITE))
        .insert_non_send_resource(PickedColorReceiver(rx))
        .add_systems(Startup, setup_ui)
        .add_systems(Update, pick.run_if(input_just_pressed(KeyCode::Space)))
        .add_systems(
            Update,
            click_update.run_if(input_just_pressed(MouseButton::Left)),
        )
        .add_systems(Update, pick_update)
        .run();
}

#[derive(Component)]
struct DisplayedHexCodeText;

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
                        DisplayedHexCodeText,
                    ));
                });
        });
}

struct PickedColorReceiver(mpsc::Receiver<Color>);

fn pick() {
    let sampler = unsafe { NSColorSampler::new() };
    let block = RcBlock::new(move |color: *mut NSColor| {
        if color.is_null() {
            return;
        }

        let color = unsafe { &*color };

        let mut red = 0.0f64;
        let mut green = 0.0f64;
        let mut blue = 0.0f64;
        let mut alpha = 0.0f64;

        unsafe {
            color.getRed_green_blue_alpha(&mut red, &mut green, &mut blue, &mut alpha);
        }

        let color = Color::srgba(red as f32, green as f32, blue as f32, alpha as f32);

        if let Some(tx) = PICK_TX.get() {
            let _ = tx.send(color);
        }
    });

    unsafe {
        sampler.showSamplerWithSelectionHandler(&block);
    }
}

fn pick_update(
    mut clear_color: ResMut<ClearColor>,
    mut displayed_hex_code_text: Single<(&mut Text, &mut TextColor), With<DisplayedHexCodeText>>,
    rx: NonSend<PickedColorReceiver>,
) {
    while let Ok(color) = rx.0.try_recv() {
        let hex = color.to_srgba().to_hex();

        clipboard::copy(&hex);

        clear_color.0 = color;
        displayed_hex_code_text.0.0 = hex;
        displayed_hex_code_text.1.0 = if color.luminance() > 0.5 {
            Color::BLACK
        } else {
            Color::WHITE
        };
    }
}

fn click_update(displayed_hex_code_text: Single<&Text, With<DisplayedHexCodeText>>) {
    let hex = &*displayed_hex_code_text.0;
    if !hex.is_empty() {
        clipboard::copy(hex);
    }
}
