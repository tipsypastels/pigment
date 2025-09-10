use bevy::prelude::*;

pub struct ValuePlugin;

impl Plugin for ValuePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ColorValue(Color::WHITE))
            .insert_resource(HexValue("#FFFFFF".to_string()));
    }
}

#[derive(Resource)]
pub struct ColorValue(pub Color);

#[derive(Resource)]
pub struct HexValue(pub String);
