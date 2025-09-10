use crate::markers::UpdateMarker;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub struct DraggablePlugin;

impl Plugin for DraggablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            on_click
                .run_if(input_just_pressed(MouseButton::Left))
                .in_set(UpdateMarker::Interaction),
        );
    }
}

fn on_click(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        window.start_drag_move();
    }
}
