use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let path = Path::new("links.txt");
    let line_num = 6usize;
    let new_content = "new content";
    append_line(path, new_content);
    let line = get_line_at(&path, line_num - 1);
    let last_line = get_last_line(&path);
    let last_content = get_line_at(&path, last_line - 1);
    println!("{}", line.unwrap());
    println!("{}", last_line);
    println!("{}", last_content.as_ref().unwrap());

    println!("the last line is {} containing the following content {}", last_line, last_content.unwrap());
 
}
fn get_line_at(path: &Path, line_num: usize) -> Result<String, Error> {
    let file = File::open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_num).expect("No line found at that position")
}

fn append_line(path: &Path, new_content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path).expect("File not found or cannot be opened");
    write!(file, "{}\n", new_content);
    Ok(())
}

fn get_last_line(path: &Path) -> usize {
    let mut file = OpenOptions::new().read(true).open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let lines = content.lines();
    return lines.count();
}