#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::{
    collections::HashMap,
    env::{self},
    fs::{self, File},
    io::{BufRead, BufReader, Result, Write},
    path::Path,
};

lazy_static! {
    static ref REG: HashMap<&'static str, Regex> = {
        let mut reg_map = HashMap::new();
        reg_map.insert(
            "extension_re",
            Regex::new(r"\.(.+)$").expect("No valid match"),
        );
        reg_map.insert(
            "line_re",
            Regex::new(r"////(.+)$").expect("No matched line"),
        );

        reg_map
    };
}

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

fn get_new_file_name(filename: &str, extension: &str) -> String {
    let filename = Path::new(filename).file_name().unwrap();
    let filename: String = filename
        .to_str()
        .unwrap()
        .to_string()
        .chars()
        .rev()
        .collect();
    let filename: String = format!(
        "{}.{}",
        if let Some(val) = REG.get("extension_re").unwrap().captures(&filename) {
            val[1].chars().rev().collect()
        } else {
            "rs".to_owned()
        },
        extension
    );
    filename
}

fn write_file(filename: &str, extension: &str, map: HashMap<String, Vec<String>>) -> Result<()> {
    if !map.keys().any(|x| x == &extension.clone().to_string()) {
        println!(
            "File Extension provided [.{}], has no corresponding write up in the file been used.",
            &extension
        );
        std::process::exit(-1);
    }

    // remove the old file, all the data therein are in the
    // map to be written to the new file with the new extension.
    fs::remove_file(filename)?;

    // get_new_file_name is a help function that gets both filename
    // and file extension to be used. And return a new filename
    // with a new extension.
    let filename = get_new_file_name(filename, extension);
    let mut file = File::create(filename).unwrap();

    // write the new file with the extension wanted first.
    if map.contains_key(extension) {
        for line in map.get(extension).unwrap() {
            // check if each line matches
            if REG.get("line_re").unwrap().is_match(line) {
                // if the lines matches, then assign the line, to variable `line`
                let line = REG.get("line_re").unwrap().captures(&line).unwrap();
                if line[1].starts_with("start") {
                    writeln!(file, "{}", &line[0])?; // take the whole line `0`
                } else {
                    writeln!(file, "{}", &line[1])?; // take only the captured match `1`
                }
            } else {
                writeln!(file, "{}", line)?;
            }
        }
    }

    // write all the other lines in the files
    // for other languages avaliable in the program
    for key in map
        .keys()
        .filter(|a| **a != extension.to_string())
        .collect::<Vec<_>>()
    {
        for line in map.get(key).unwrap() {
            if line.starts_with("////") || line.contains("////") {
                writeln!(file, "{}", line)?;
            } else {
                writeln!(file, "////{}", line)?;
            }
        }
    }

    Ok(())
}

fn read_file(filename: &str) -> Result<HashMap<String, Vec<String>>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let file = File::open(filename.to_owned())?;
    let buf = BufReader::new(file);
    let mut key = String::with_capacity(10);
    let mut store = vec![];

    for line in buf.lines() {
        let line = line?;
        if line.trim().starts_with("////start") {
            // use regex here later
            let keys = line.split_whitespace().into_iter().collect::<Vec<_>>();
            key = keys[1].to_string();
            key.shrink_to_fit();
            store = vec![line.clone()];
        } else {
            store.push(line.clone());
        }
        map.insert(key.clone(), store.clone());
    }
    map.insert(key, store);
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_new_file_name() {
        assert_eq!(
            "hello.go",
            get_new_file_name(/*filename*/ "hello.rs", /*file extension*/ "go")
        );
    }

    #[test]
    fn test_get_new_file_name_with_several_ext() {
        assert_eq!(
            "hello.copy.new.dart",
            get_new_file_name(
                /*filename*/ "hello.copy.new.c",
                /*file extension*/ "dart"
            ),
            "It changes only the last extension for the file"
        );
    }
}
