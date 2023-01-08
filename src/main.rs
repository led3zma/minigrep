use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {} in file {}", query, file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Error while trying to read the file :(");

    println!("With text:\n{contents}");
}
