use rexif::ExifTag;
use std::env;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;

fn main() {
    let dir_path = env::current_dir().unwrap();

    // Only iterate over files in the current directory, not descending into subdirectories
    let walker = WalkDir::new(&dir_path).max_depth(1);

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            match extension.as_str() {
                "jpg" | "jpeg" => handle_jpeg_and_corresponding_raf(path, &dir_path),
                "mov" => handle_mov_file(path, &dir_path),
                _ => (),
            }
        }
    }

    println!("File organization complete.");
}

fn handle_jpeg_and_corresponding_raf(path: &Path, dir_path: &Path) {
    if let Ok(exif_data) = rexif::parse_file(path) {
        if let Some(tag) = exif_data
            .entries
            .iter()
            .find(|tag| tag.tag == ExifTag::DateTimeOriginal)
        {
            let date = tag
                .value_more_readable
                .split_whitespace()
                .next()
                .unwrap_or("")
                .replace(":", "-");
            if !date.is_empty() {
                move_file_to_date_directory(path, dir_path, &date);

                let raf_path = path.with_extension("RAF");
                if raf_path.exists() {
                    move_file_to_date_directory(&raf_path, dir_path, &date);
                }
            }
        }
    } else {
        eprintln!("Failed to read EXIF data for {:?}.", path.display());
    }
}

fn handle_mov_file(path: &Path, dir_path: &Path) {
    if let Ok(metadata) = path.metadata() {
        if let Ok(modified_time) = metadata.modified() {
            if let Ok(duration_since_epoch) = modified_time.duration_since(UNIX_EPOCH) {
                let date =
                    chrono::DateTime::from_timestamp(duration_since_epoch.as_secs() as i64, 0)
                        .unwrap()
                        .format("%Y-%m-%d")
                        .to_string();
                // Check if the filename already starts with the date to avoid duplicate prefix
                if !path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with(&date)
                {
                    move_file_to_date_directory(path, dir_path, &date);
                }
            }
        }
    }
}

fn move_file_to_date_directory(path: &Path, dir_path: &Path, date: &str) {
    let new_dir_path = dir_path.join(date);
    if let Err(e) = fs::create_dir_all(&new_dir_path) {
        eprintln!(
            "Error creating directory {:?}: {}",
            new_dir_path.display(),
            e
        );
        return;
    }
    let file_name = path.file_name().unwrap().to_str().unwrap();
    // Avoid adding the date prefix if it already exists
    let new_file_name = if !file_name.starts_with(date) {
        format!("{}-{}", date, file_name)
    } else {
        file_name.to_string()
    };
    let new_file_path = new_dir_path.join(new_file_name);
    if let Err(e) = fs::rename(path, &new_file_path) {
        eprintln!("Error moving file to {:?}: {}", new_file_path.display(), e);
    }
}
