mod file;
mod search_options;

use std::{env::args, fs::{self}, path::Path, process};

use file::FileSize;
use search_options::SearchOptions;

fn yellow(text: &str) -> String {
    format!("\x1b[92m{}\x1b[0m",text)
}

fn print_file_size(file_size: &FileSize, separator: char) {
    let decorate_char = '│';
    let (size, unit_of_measurement) = to_human_format(file_size.size);
    let margin = format!("  {} ", decorate_char).repeat(file_size.depth);
    let mut short_path = file_size.path.to_string().split('/').collect::<Vec<&str>>()[file_size.depth + 1..].join("/");

    if file_size.is_dir {
        short_path.push_str("/");
    }

    let formatted_size = yellow(&format!("{:.0} {}", size, unit_of_measurement));

    println!("{margin}{short_path}{}{formatted_size}",yellow(&separator.to_string()));
}

fn to_human_format(value: usize) -> (f64, String) {
    let unit_of_measurement = ["B", "KB", "MB", "GB"];
    let mut inner_value: f64 = value as f64;

    let mut index = 0;

    loop {
        if (inner_value / 1024.0).round() > 0.0 && index < unit_of_measurement.len() - 1 {
            inner_value = inner_value / 1024.0;
            index += 1;
        } else {
            return (inner_value, unit_of_measurement[index].to_string());
        }
    }
}

fn get_file_size(path: &str) -> usize {
    let file_content = fs::read(path).unwrap_or_else(|error| match error.kind() {
        std::io::ErrorKind::NotFound => {
            eprintln!("File '{}' not found", path);
            process::exit(1);
        }
        std::io::ErrorKind::PermissionDenied => {
            eprintln!("Permission denied '{}'", path);
            process::exit(1);
        }
        error => {
            eprintln!("{}", error);
            process::exit(1);
        }
    });

    let file_size = file_content.len();

    file_size
}

fn get_size(
    path: &str,
    search_options: &SearchOptions,
    depth: usize,
    sizes: &mut Vec<FileSize>,
) -> usize {
    let mut size: usize = 0;

    let is_dir = Path::new(path).is_dir();

    if is_dir {
        let dir_path = path;

        let files_from_dir = fs::read_dir(&dir_path).unwrap_or_else(|error| match error.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("Dir {} not found", dir_path);
                process::exit(1);
            }
            std::io::ErrorKind::PermissionDenied => {
                eprintln!("Dir {} permission denied", dir_path);
                process::exit(1);
            }
            error => {
                eprintln!("{}", { error });
                process::exit(1);
            }
        });

        for item_result in files_from_dir {
            let item = item_result.unwrap_or_else(|error| match error.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("Dir {} not found", dir_path);
                    process::exit(1);
                }
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!("Dir {} permission denied", dir_path);
                    process::exit(1);
                }
                error => {
                    eprintln!("{}", { error });
                    process::exit(1);
                }
            });

            size += get_size(
                item.path().to_str().unwrap(),
                search_options,
                depth + 1,
                sizes,
            );
        }


        sizes.push(FileSize::new(path.to_string(),size,depth, true));

        return size;
    } else {
        let file_size = get_file_size(path);

        sizes.push(FileSize::new(path.to_string(), file_size, depth, false));

        return file_size;
    }
}

fn main() {
    let arguments: Vec<String> = args().collect();

    let path = arguments.get(arguments.len() - 1).unwrap_or_else(|| {
        eprintln!("File name not found");
        process::exit(1);
    });

    let search_options = SearchOptions::from(&arguments);

    let mut folder_files: Vec<FileSize> = vec![];

    get_size(&path, &search_options, 0, &mut folder_files);

    folder_files.reverse();

    if search_options.show_all {
        for file_size in folder_files {
            print_file_size(&file_size,'・');
        }
    } else {
        let file = &folder_files[0];
        print_file_size(file, ' ');
    }
}
