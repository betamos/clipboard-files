use clipboard_win::{formats, get_clipboard};

use crate::Error;

pub(crate) fn read_clipboard() -> Result<Vec<String>, Error> {
    get_clipboard(formats::FileList {}).map_err(|_| Error::NoFiles)
}

// pub fn read_clipboard() -> Result<Vec<String>, Error> {
//     let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
//     let filelist = formats::FileList{};
//     let mut out: Vec<String> = Vec::new();
//     // this unwrap fails if not files
//     let n = filelist.read_clipboard(&mut out).unwrap();
//     Ok(out)
// }
