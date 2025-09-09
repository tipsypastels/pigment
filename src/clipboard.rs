use objc2::runtime::ProtocolObject;
use objc2_app_kit::{NSPasteboard, NSPasteboardWriting};
use objc2_foundation::{NSArray, NSString};

pub fn copy(s: &str) {
    let pasteboard = unsafe { NSPasteboard::generalPasteboard() };
    let string = NSString::from_str(s);
    let string_protocol = ProtocolObject::<dyn NSPasteboardWriting>::from_retained(string);
    let array = NSArray::from_retained_slice(&[string_protocol]);

    unsafe {
        pasteboard.clearContents();
        pasteboard.writeObjects(&array);
    }
}
