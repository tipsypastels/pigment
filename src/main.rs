use bevy::{
    input::common_conditions::input_just_pressed, log::LogPlugin, prelude::*,
    window::WindowResolution,
};
use block2::RcBlock;
use objc2::runtime::ProtocolObject;
use objc2_app_kit::{NSColor, NSColorSampler, NSPasteboard, NSPasteboardWriting};
use objc2_foundation::{NSArray, NSString};
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
                        resolution: WindowResolution::new(200.0, 200.0),
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
        .add_systems(Update, pick_update)
        .run();
}

#[derive(Component)]
struct DisplayedHexCodeText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text::new(""),
                TextFont {
                    font: asset_server.load("fonts/Comfortaa.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                DisplayedHexCodeText,
            ));
        });
}

struct PickedColorReceiver(mpsc::Receiver<Color>);

fn pick() {
    let sampler = unsafe { NSColorSampler::new() };
    let block = RcBlock::new(move |color: *mut NSColor| {
        if color.is_null() {
            return;
        }

        println!("Called the block");

        let color = unsafe { &*color };

        let mut red = 0.0f64;
        let mut green = 0.0f64;
        let mut blue = 0.0f64;
        let mut alpha = 0.0f64;

        unsafe {
            color.getRed_green_blue_alpha(&mut red, &mut green, &mut blue, &mut alpha);
        }

        println!("Got the color");

        let color = Color::srgba(red as f32, green as f32, blue as f32, alpha as f32);

        if let Some(tx) = PICK_TX.get() {
            println!("Sending the color");
            let _ = tx.send(color);
            println!("Sent the color");
        }
    });

    unsafe {
        sampler.showSamplerWithSelectionHandler(&block);
    }
}

fn pick_update(
    mut clear_color: ResMut<ClearColor>,
    mut displayed_hex_code_text: Single<&mut Text, With<DisplayedHexCodeText>>,
    rx: NonSend<PickedColorReceiver>,
) {
    while let Ok(color) = rx.0.try_recv() {
        println!("Received the color");

        let hex = color.to_srgba().to_hex();

        copy_to_clipboard(&hex);

        clear_color.0 = color;
        displayed_hex_code_text.0 = hex;
    }
}

fn copy_to_clipboard(s: &str) {
    let pasteboard = unsafe { NSPasteboard::generalPasteboard() };

    unsafe {
        pasteboard.clearContents();
    }

    let string = NSString::from_str(s);
    let string_protocol = ProtocolObject::<dyn NSPasteboardWriting>::from_retained(string);
    let array = NSArray::from_retained_slice(&[string_protocol]);

    unsafe {
        pasteboard.writeObjects(&array);
    }
}
