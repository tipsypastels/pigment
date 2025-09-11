use bevy::prelude::*;
use objc2_app_kit::NSEventModifierFlags;
use objc2_foundation::ns_string;
use std::{cell::UnsafeCell, marker::PhantomData};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(EnsureNonSend(PhantomData))
            .add_systems(Startup, setup_menu);
    }
}

struct EnsureNonSend(PhantomData<UnsafeCell<()>>);

fn setup_menu(_: NonSend<EnsureNonSend>) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSMenu, NSMenuItem};
    use objc2_foundation::ns_string;

    let mtm = MainThreadMarker::new().unwrap();
    let app = NSApplication::sharedApplication(mtm);
    let main_menu = unsafe { app.mainMenu() };
    let Some(main_menu) = main_menu else {
        return;
    };

    let color_menu_item = NSMenuItem::new(mtm);

    unsafe {
        color_menu_item.setTitle(ns_string!("Color"));
    }

    main_menu.addItem(&color_menu_item);

    let color_menu = NSMenu::new(mtm);

    unsafe {
        color_menu.setTitle(ns_string!("Color"));
    }

    let color_menu_item_pick = unsafe {
        let item = NSMenuItem::initWithTitle_action_keyEquivalent(
            mtm.alloc(),
            ns_string!("Pick"),
            None,
            ns_string!("Space"),
        );
        item.setKeyEquivalentModifierMask(NSEventModifierFlags::empty());
        item
    };

    color_menu.addItem(&color_menu_item_pick);

    color_menu_item.setSubmenu(Some(&color_menu));

    dbg!(unsafe { app.mainMenu() });
    dbg!(color_menu);
    dbg!(color_menu_item);
}
