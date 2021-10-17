use gdnative::prelude::*;

mod rust_entry;

use rust_entry::RustEntry;

fn init(handle: InitHandle) {
    handle.add_class::<RustEntry>();
}

godot_init!(init);
