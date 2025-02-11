use std::env;
use std::path::Path;
use log::{info, warn};
use env_logger;
use chrono::{DateTime, Local};
use home_dir::HomeDirExt;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let download_dir = Path::new("~/Downloads/cleaner_test").expand_home().unwrap();
    let now = chrono::Local::now();
    info!("Starting cleaner at: {}", now);

    for entry in download_dir.read_dir().unwrap() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                warn!("Error reading entry: {}", e);
                continue;
            }
        };
        let path = entry.path();

        let modified:DateTime<Local> = entry.metadata().unwrap().modified().unwrap().into();
        let diff = now - modified;
        if diff.num_hours() > 24 {
            info!("Deleting entry: {:?}", path);
            if path.is_dir() {
                if let Err(e) = std::fs::remove_dir_all(&path) {
                    warn!("Error deleting directory: {}", e);
                }
            } else if path.is_file() {
                if let Err(e) = std::fs::remove_file(&path) {
                    warn!("Error deleting file: {}", e);
                }
            } else {
                warn!("Unknown entry type: {:?}", path);
            }
        }

    }

}
