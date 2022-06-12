use std::path::PathBuf;

#[cfg(target_os = "linux")]
use gtk::{gdk::SELECTION_CLIPBOARD, Clipboard};

#[cfg(target_os = "windows")]
use clipboard_win::{Clipboard, formats, Getter, get_clipboard};

#[derive(Debug)]
pub enum Error {
    NoFiles,
}

fn main() {
    println!("Hello, world!");
    let files = get_files_windows().unwrap();
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

#[cfg(target_os = "windows")]
fn get_files_windows() -> Result<Vec<String>, Error> {
    get_clipboard(formats::FileList{}).map_err(|_| Error::NoFiles)
    // let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
    // let filelist = formats::FileList{};
    // let mut out: Vec<String> = Vec::new();
    // // this unwrap fails if not files
    // let n = filelist.read_clipboard(&mut out).unwrap();
    // Ok(out)
}

