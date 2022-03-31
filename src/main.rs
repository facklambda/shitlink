use tokio_uring::fs::File;
use tokio_uring::fs::OpenOptions;
use tokio::io::BufReader;
use std::path::Path;
 
fn main() {
    let path = Path::new("links.txt");
    let line_num = 7usize;
    let line = get_line_at(&path, line_num - 1);
    println!("{}", line.unwrap());
}
 
fn get_line_at(path: &Path, line_num: usize) -> Result<(), Box<dyn std::error::Error>> {
    tokio_uring::start(async {        
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .await?;
        let content = BufReader::new(&file);
        let mut lines = content.lines();
        lines.nth(line_num).expect("No line found at that position");
        
        Ok(())
    })    

}
