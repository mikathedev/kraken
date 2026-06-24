
mod download;
mod scrape;
mod tools;

#[tauri::command]
async fn get_shows() -> Vec<String> {
    let _ = scrape::scrape("show_name".to_string(), "https://a.111477.xyz/tvs/A%20Haunting/".to_string()).await;
    let data = tools::get_show_data();
    let mut shows: Vec<String> = Vec::new();
    for (show, _) in data {
        shows.push(show);
    }
    shows
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_shows])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
