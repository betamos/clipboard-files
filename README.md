## Clipboard Files

This crate lets you read file paths from the system wide clipboard, that are copied from
Explorer, Finder, etc.

It's supported on Windows, Linux (using GTK) and MacOS.

## Reading

```
use clipboard_files;

fn main() {
    let files = clipboard_files::read();
    println!(files);
}
```

## Writing

Not supported, mostly due to lack of support in the Linux and Windows upstream crates.
Consider filing a PR in those.

## Why?

There are several clipboard crates, for instance https://github.com/1Password/arboard.
That crate is supported in multiple unix-like environments because it talks X11 directly.
This crate uses the GTK bindings for Linux, which offers a much simpler API.

Ideally, all upstream crates should support files. When they do, we'd be better off deleting
this one. In the meantime, use this crate.