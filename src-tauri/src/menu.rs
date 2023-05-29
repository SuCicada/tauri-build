use tauri::{Manager, WindowMenuEvent};

// #[cfg(target_os = "macos")]
// use tauri::AboutMetadata;


// --- Menu Event
pub fn menu_handler(event: WindowMenuEvent<tauri::Wry>) {
    let win = Some(event.window()).unwrap();
    let app = win.app_handle();
    let menu_id = event.menu_item_id();
    // let menu_handle = win.menu_handle();

    match menu_id {
        // "check_update" => {
        //   utils::run_check_update(app, false, None);
        // }
        "zoom_0" => win.eval("window.__zoom0 && window.__zoom0()").unwrap(),
        "zoom_out" => win.eval("window.__zoomOut && window.__zoomOut()").unwrap(),
        "zoom_in" => win.eval("window.__zoomIn && window.__zoomIn()").unwrap(),
        "reload" => win.eval("window.location.reload()").unwrap(),
        "go_back" => win.eval("window.history.go(-1)").unwrap(),
        "go_forward" => win.eval("window.history.go(1)").unwrap(),
        "scroll_top" => win.eval(
            r#"window.scroll({
          top: 0,
          left: 0,
          behavior: "smooth"
          })"#,
        ).unwrap(),
        "scroll_bottom" => win.eval(
            r#"window.scroll({
          top: document.body.scrollHeight,
          left: 0,
          behavior: "smooth"})"#,
        ).unwrap(),
        // Help
        "dev_tools" => {
            win.open_devtools();
            win.close_devtools();
        },
        "restart" => tauri::api::process::restart(&app.env()),

        _ => (),
    }
}

