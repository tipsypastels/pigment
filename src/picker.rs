use bevy::prelude::*;
use std::sync::{OnceLock, mpsc};

static TX: OnceLock<mpsc::Sender<Color>> = OnceLock::new();

pub fn sender(tx: mpsc::Sender<Color>) {
    let _ = TX.set(tx);
}

pub struct PickerPlugin;

impl Plugin for PickerPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
