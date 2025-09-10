use crate::markers::UpdateMarker;
use bevy::prelude::*;

pub struct ValuePlugin;

impl Plugin for ValuePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ColorValue(Color::WHITE))
            .insert_resource(HexValue("#FFFFFF".to_string()))
            .add_systems(Update, set_hex_from_color.in_set(UpdateMarker::Logic));
    }
}

#[derive(Resource)]
pub struct ColorValue(pub Color);

#[derive(Resource)]
pub struct HexValue(pub String);

fn set_hex_from_color(color: Res<ColorValue>, mut hex: ResMut<HexValue>) {
    if color.is_changed() {
        hex.0 = color.0.to_srgba().to_hex();
    }
}
