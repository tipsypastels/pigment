use crate::{markers::UpdateMarker, value::ColorValue};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use std::sync::{OnceLock, mpsc};

static TX: OnceLock<mpsc::Sender<Color>> = OnceLock::new();

pub struct PickerPlugin;

impl Plugin for PickerPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = mpsc::channel();
        let _ = TX.set(tx);

        app.insert_non_send_resource(Rx(rx))
            .add_systems(
                Update,
                pick.run_if(input_just_pressed(KeyCode::Space))
                    .in_set(UpdateMarker::Interaction),
            )
            .add_systems(Update, on_pick.in_set(UpdateMarker::Logic));
    }
}

struct Rx(mpsc::Receiver<Color>);

fn pick() {
    use block2::RcBlock;
    use objc2_app_kit::{NSColor, NSColorSampler};

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

        if let Some(tx) = TX.get() {
            let _ = tx.send(color);
        }
    });

    unsafe {
        sampler.showSamplerWithSelectionHandler(&block);
    }
}

fn on_pick(mut color_value: ResMut<ColorValue>, rx: NonSend<Rx>) {
    while let Ok(color) = rx.0.try_recv() {
        color_value.0 = color;
    }
}
