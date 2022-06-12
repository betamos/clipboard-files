use gtk::{gdk::SELECTION_CLIPBOARD, Clipboard};

use crate::Error;

fn uri_to_string(uri: &str) -> Option<String> {
    urlencoding::decode(uri)
        .ok()?
        .strip_prefix("file://")
        .map(String::from)
}

pub(crate) fn read_clipboard() -> Result<Vec<String>, Error> {
    gtk::init().map_err(|err| Error::SystemError(err.to_string()))?;
    let cb = Clipboard::get(&SELECTION_CLIPBOARD);
    let paths = cb.wait_for_uris();
    if paths.is_empty() {
        return Err(Error::NoFiles);
    }
    Ok(paths
        .into_iter()
        .filter_map(|path| uri_to_string(path.as_str()))
        .collect())
}
