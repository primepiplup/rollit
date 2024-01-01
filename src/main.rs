use dirs::home_dir;
use parser::Args;
use rand::{thread_rng, Rng};
use std::env::args;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

pub mod parser;

static ROLL_FILES: &str = ".local/share/rollit/";

fn main() {
    let args: Vec<String> = args().collect();
    let arg_count = args.len();

    if arg_count <= 1 {
        println!("Usage:");
        println!("rollit [filename]\n");
        println!("Available files: ");
        let filenames = get_filenames();
        for name in filenames {
            println!(" - {}", name);
        }
        return ();
    }

    let parsed_args = Args::parse(args);

    let filepath = compose_filepath(&parsed_args.filename);
    let mut counter: u16 = 0;
    while counter < parsed_args.count {
        display_random_line_from_file(&filepath);
        counter = counter + 1;
    }
}

fn get_roll_files_folder() -> PathBuf {
    let mut filepath = home_dir().expect("No home directory found.");
    filepath.push(ROLL_FILES);
    return filepath;
}

fn get_filenames() -> Vec<String> {
    let filepath = get_roll_files_folder();
    let files = read_dir(filepath).unwrap();
    files
        .into_iter()
        .map(|file| file.unwrap().path().to_str().unwrap().to_string())
        .collect()
}

fn compose_filepath(filename: &String) -> PathBuf {
    let mut filepath = get_roll_files_folder();
    filepath.push(filename);
    return filepath;
}

fn display_random_line_from_file(filepath: &PathBuf) {
    let filecontent: String = read_to_string(filepath).expect("File not found.");
    let mut file_lines = filecontent.lines();
    let line_count = file_lines.clone().count();
    let selection_index = thread_rng().gen_range(0..line_count);
    let selected_line = file_lines
        .nth(selection_index)
        .expect("Line not found at index.");
    println!("{}", selected_line);
}
