use std::fs::File;
use std::path::Path;
use std::io;
use std::io::prelude::*;

pub fn read_file(pathstr: &str) -> io::Result<String> {
    let mut file: File = File::open(Path::new(pathstr))?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_lines(pathstr: &str) -> io::Result<Vec<String>> {
    read_file(pathstr).map(|s: String| {
        s.split("\n")
            .map(String::from)
            .collect()
    })
}
