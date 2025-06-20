use ::plyg::plyg::{read_file, write_file};

use std::{
    env::{self},
    path::Path,
};

fn main() {
    let cli_options: Vec<_> = env::args().skip(1).collect();
    if cli_options.is_empty() {
        println!("Usage: ./rite <programming_language_to_write> <filename>");
        std::process::exit(-1);
    }

    let filename = cli_options[1].clone();
    let path = Path::new(&filename);
    if path.exists() {
        write_file(
            &filename.clone(),
            &cli_options[0],
            read_file(&filename).unwrap(),
        )
        .expect("can't write file");
    } else {
        println!("{:?} doesn't exit. Use a valid filename", filename);
        std::process::exit(-1);
    }
}
