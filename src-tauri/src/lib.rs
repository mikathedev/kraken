#[derive(serde::Serialize)]
struct Show {
    title: String,
    season: u32,
    episode: u32,
}

#[tauri::command]
fn get_shows() -> Vec<Show> {
    let mut shows: Vec<Show> = Vec::new();
    shows.push(Show {
        title: "".to_string(),
        season: 0,
        episode: 0,
    });
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
