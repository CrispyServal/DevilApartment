use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct RustEntry;

#[methods]
impl RustEntry {
    fn new(_owner: &Node) -> Self {
        RustEntry
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Hello, world.");
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<RustEntry>();
}

godot_init!(init);