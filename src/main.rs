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
    let line = get_line_at(&path, line_num - 1);
    let 
    println!("{}", line.unwrap());
}
 
fn get_line_at(path: &Path, line_num: usize) -> Result<String, Error> {
    let file = File::open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_num).expect("No line found at that position")
}

fn append(path: &Path, new_content: &str) -> Result<usize, Error> {
    let mut file = OpenOptions::new().append(true).open(path).expect("File not found or cannot be opened");
    //fs::write(&file, new_content)?;
    write!(file, "{}", new_content)?;

    let content = BufReader::new(&file);
    let mut lines = content.lines();
    return Ok(lines.count());

}