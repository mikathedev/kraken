use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Show {
    episode: u32,
    season: u32,
    #[serde(default)]
    urls: Option<HashMap<u32, HashMap<u32, String>>>
}


pub fn get_show_data() -> HashMap<String, Show> {
    let path = Path::new("/home/mika/.config/shows.json");
    if !path.exists() {
        if let Err(e) = File::create(path).and_then(|mut f| std::io::Write::write_all(&mut f, b"{}")) {
            println!("Failed to create shows.json: {e}");
            return HashMap::new();
        }
    }
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open shows.json: {e}");
            return HashMap::new();
        }
    };
    let mut json = String::new();
    if let Err(e) = file.read_to_string(&mut json) {
        println!("Failed to read shows.json: {e}");
        return HashMap::new();
    }
    if json.trim().is_empty() {
        return HashMap::new();
    }
    match serde_json::from_str(&json) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to parse shows.json: {e}");
            HashMap::new()
        }
    }
}
