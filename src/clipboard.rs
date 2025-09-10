use crate::{markers::UpdateMarker, value::HexValue};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub struct ClipboardPlugin;

impl Plugin for ClipboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            // TODO: Ctrl-C?
            copy.run_if(input_just_pressed(KeyCode::KeyC))
                .in_set(UpdateMarker::Interaction),
        );
    }
}

fn copy(hex_value: Res<HexValue>) {
    copy_impl(&hex_value.0);
}

fn copy_impl(s: &str) {
    use objc2::runtime::ProtocolObject;
    use objc2_app_kit::{NSPasteboard, NSPasteboardWriting};
    use objc2_foundation::{NSArray, NSString};

    let pasteboard = unsafe { NSPasteboard::generalPasteboard() };
    let string = NSString::from_str(s);
    let string_protocol = ProtocolObject::<dyn NSPasteboardWriting>::from_retained(string);
    let array = NSArray::from_retained_slice(&[string_protocol]);

    unsafe {
        pasteboard.clearContents();
        pasteboard.writeObjects(&array);
    }
}
