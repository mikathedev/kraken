
fn get_show_data() {
    let json = "";
    let data: serde_json::Value = serde_json::from_str(json).unwrap();
    
    let shows: Vec<String> = data["shows"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();
    println!("{:?}", shows);
}

#[tauri::command]
fn get_shows() -> Vec<String> {
    let mut shows: Vec<String> = Vec::new();
    
    shows
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    get_show_data();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_shows])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
