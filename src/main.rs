use std::path::PathBuf;

#[cfg(target_os = "linux")]
use gtk::{gdk::SELECTION_CLIPBOARD, Clipboard};

#[derive(Debug)]
pub enum Error {
    NoFiles,
}

fn main() {
    println!("Hello, world!");
    let files = get_files_linux().unwrap();
    println!("files: {:?}", files);
}

#[cfg(target_os = "linux")]
fn get_files_linux() -> Result<Vec<String>, Error> {
    gtk::init().unwrap();
    let cb = Clipboard::get(&SELECTION_CLIPBOARD);
    let paths = cb.wait_for_uris();
    if paths.len() == 0 {
        return Err(Error::NoFiles);
    }
    println!("{} entries:", paths.len());
    Ok(paths
        .into_iter()
        .map(|path| urlencoding::decode(&path.as_str()).unwrap().into_owned())
        .collect())
}
