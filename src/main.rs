use notify_rust::Notification;
use std::{collections::HashSet, fs, time::Duration};
use tokio::time::interval;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path_str = fs::read_to_string("/home/chronois/.config/folder_monitoring.txt")?;
    let path = path_str.trim().to_string();

    let mut interval = interval(Duration::from_millis(100));

    let path_clone = path.clone();
    let mut folder_files_old: HashSet<String> =
        tokio::task::spawn_blocking(move || check_folder(&path_clone)).await??;

    loop {
        interval.tick().await;

        let path_clone = path.clone();
        let current_files =
            tokio::task::spawn_blocking(move || check_folder(&path_clone)).await??;

        for new_file in current_files.difference(&folder_files_old) {
            Notification::new()
                .summary("New File in!")
                .body(&format!("Added new file: {}", new_file))
                .appname("Folder Monitor")
                .show()?;
        }

        folder_files_old = current_files;
    }
}

fn check_folder(path: &str) -> Result<HashSet<String>, std::io::Error> {
    let mut folder_files = HashSet::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().into_owned();
        folder_files.insert(file_name);
    }

    Ok(folder_files)
}
