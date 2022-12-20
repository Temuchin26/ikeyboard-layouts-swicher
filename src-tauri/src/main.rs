#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use rdev::{listen, Event, EventType, Key};
use serde;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::{GlobalShortcutManager, Manager};

mod utils;

#[derive(Clone, serde::Serialize)]
struct CurrentLayoutChangePayload {
    currentLayout: String,
}

#[derive(Clone, serde::Serialize)]
struct LayoutsChangePayload {
    layouts: String,
}

fn main() {
    let layouts_shared = Arc::new(Mutex::new(vec!["us", "ru", "ua"]));
    let current_layout_shared = Arc::new(Mutex::new("us"));

    tauri::Builder::default()
        .setup(move |app| {
            let handle = app.handle();
            let main_window = app.get_window("main").unwrap();
            let main_window_copy = app.get_window("main").unwrap();
            
            let layout_clone1 = layouts_shared.clone();
            let layout_clone2 = layouts_shared.clone();
            let current_layout_clone1 = current_layout_shared.clone();
            let current_layout_clone2 = current_layout_shared.clone();

            std::thread::spawn(move || {
                if let Err(error) = listen(move |event: Event| {
                    if event.event_type == EventType::KeyRelease(Key::MetaLeft) {
                        let mut layouts = layout_clone1.lock().unwrap();
                        let current_layout = current_layout_clone1.lock().unwrap();

                        let current_layout_index_after_update =
                            utils::get_current_layout_index(&current_layout, &layouts);
                        layouts.remove(current_layout_index_after_update);
                        layouts.insert(0, current_layout.clone());

                        utils::apply_layouts(current_layout);

                        let layouts_copy = &*layouts;
                        let json = serde_json::to_string(layouts_copy).unwrap();
                        match main_window_copy.emit(
                            "hide-window",
                            LayoutsChangePayload {
                                layouts: String::from(""),
                            },
                        ) {
                            Ok(_) => main_window_copy.hide().expect("Failed to hide window"),
                            Err(_) => panic!("Failed to emit layouts-change"),
                        }
                    }
                }) {
                    println!("Error: {:?}", error)
                }
            });

            handle
                .global_shortcut_manager()
                .register("Super+Space", move || {
                    let layouts = layout_clone2.lock().unwrap();
                    let is_window_visible = main_window
                        .is_visible()
                        .expect("Failed to fetch is visible");
                    if is_window_visible == false {
                        match main_window.show() {
                            Ok(_) => {
                                let json = serde_json::to_string(&*layouts).unwrap();

                                main_window
                                    .emit("layouts-change", LayoutsChangePayload { layouts: json })
                                    .unwrap();
                            }
                            Err(_) => panic!("Failed to open window"),
                        }
                    }

                    let mut current_layout = current_layout_clone2.lock().unwrap();
                    let current_layout_index =
                        utils::get_current_layout_index(&current_layout, &layouts);

                    if (current_layout_index + 1) == layouts.len() {
                        *current_layout = layouts[0];
                    } else {
                        *current_layout = layouts[current_layout_index + 1];
                    }

                    main_window
                        .emit(
                            "current-layout-change",
                            CurrentLayoutChangePayload {
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
