use dirs::home_dir;
use rand::{thread_rng, Rng};
use std::env::args;
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = args().collect();
    let arg_count = args.len();

    if arg_count <= 1 {
        println!("Usage:");
        println!("rollit [filename]");
    }

    if arg_count > 1 {
        let filepath = compose_filepath(&args[1]);
        display_random_line_from_file(filepath);
    }
}

fn compose_filepath(filename: &String) -> PathBuf {
    let mut filepath = home_dir().expect("No home directory found.");
    filepath.push(".local/share/rollit/");
    filepath.push(filename);
    return filepath;
}

fn display_random_line_from_file(filepath: PathBuf) {
    let filecontent: String = read_to_string(filepath).expect("File not found.");
    let mut file_lines = filecontent.lines();
    let line_count = file_lines.clone().count();
    let selection_index = thread_rng().gen_range(0..line_count);
    let selected_line = file_lines
        .nth(selection_index)
        .expect("Line not found at index.");
    println!("{}", selected_line);
}
