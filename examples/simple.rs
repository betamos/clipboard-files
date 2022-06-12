use clipboard_files::read;

fn main() {
    let files = read();
    println!("files (1): {:?}", files);
    let files = read();
    println!("files (2): {:?}", files);
}
