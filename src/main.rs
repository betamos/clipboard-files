use gtk::{gdk::SELECTION_CLIPBOARD, Clipboard};

#[cfg(target_os = "linux")]
fn main() {
    println!("Hello, world!");
    gtk::init().unwrap();
    let cb = Clipboard::get(&SELECTION_CLIPBOARD);
    let paths = cb.wait_for_uris();
    println!("{} entries:", paths.len());
    for path in paths {
        println!("{}", path);
    }
}
