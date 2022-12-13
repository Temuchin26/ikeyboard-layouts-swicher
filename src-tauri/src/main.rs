#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use once_cell::sync::OnceCell;
use rdev::{listen, Event, EventType, Key};
use serde;
use std::sync::Mutex;
use tauri::{GlobalShortcutManager, Manager};

mod utils;

#[derive(Clone, serde::Serialize)]
struct Payload {
    layouts: String,
    currentLayout: String,
}

static LAYOUTS: OnceCell<Mutex<Vec<&str>>> = OnceCell::new();
static CURRENT_LAYOUT: OnceCell<Mutex<&str>> = OnceCell::new();

fn main() {
    LAYOUTS
        .set(Mutex::new(vec!["us", "ru", "ua"]))
        .expect("Couldn't set LAYOUTS");
    CURRENT_LAYOUT
        .set(Mutex::new("us"))
        .expect("Couldn't set CURRENT_LAYOUT");
    tauri::Builder::default()
        .setup(move |app| {
            let handle = app.handle();
            let main_window = app.get_window("main").unwrap();
            let main_window_copy = app.get_window("main").unwrap();

            std::thread::spawn(move || {
                // This will block.
                if let Err(error) = listen(move |event: Event| {
                    if event.event_type == EventType::KeyRelease(Key::MetaLeft) {
                        let mut layouts = LAYOUTS
                            .get()
                            .expect("Can't retrieve LAYOUTS")
                            .lock()
                            .unwrap();
                        let current_layout = CURRENT_LAYOUT
                            .get()
                            .expect("Can't retriave CURRENT_LAYOUT")
                            .lock()
                            .unwrap();

                        let current_layout_index_after_update =
                            utils::get_current_layout_index(&current_layout, &layouts);
                        layouts.remove(current_layout_index_after_update);
                        layouts.insert(0, current_layout.clone());

                        utils::apply_layouts(current_layout);

                        main_window_copy.hide().expect("Failed");
                    }
                }) {
                    println!("Error: {:?}", error)
                }
            });

            handle
                .global_shortcut_manager()
                .register("Super+Space", move || {
                    let is_window_visible = main_window
                        .is_visible()
                        .expect("Failed to fetch is visible");
                    if is_window_visible == false {
                        main_window.show().expect("Failed");
                    }

                    let layouts = LAYOUTS
                        .get()
                        .expect("Can't retrieve LAYOUTS")
                        .lock()
                        .unwrap();
                    let mut current_layout = CURRENT_LAYOUT
                        .get()
                        .expect("Can't retriave CURRENT_LAYOUT")
                        .lock()
                        .unwrap();
                    let current_layout_index =
                        utils::get_current_layout_index(&current_layout, &layouts);

                    if (current_layout_index + 1) == layouts.len() {
                        *current_layout = layouts[0];
                    } else {
                        *current_layout = layouts[current_layout_index + 1];
                    }

                    let layouts_copy = &*layouts;
                    let json = serde_json::to_string(layouts_copy).unwrap();

                    handle
                        .emit_all(
                            "layoutChanged",
                            Payload {
                                layouts: json,
                                currentLayout: String::from(*current_layout),
                            },
                        )
                        .unwrap();
                })
                .expect("Failed registering shortcut");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // keep intact
}
