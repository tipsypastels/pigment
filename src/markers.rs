use bevy::prelude::*;

#[derive(Component)]
pub struct HexTextMarker;

#[derive(Component)]
pub struct PickerControlTextMarker;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateMarker {
    Interaction,
    Logic,
    Ui,
}
