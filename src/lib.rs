use gdnative::api::RigidBody;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(RigidBody)]
pub struct BugTest {}

#[methods]
impl BugTest {
    #[export]
    fn _ready(&self, _owner: &RigidBody) {
        godot_print!("hello movement!");
    }
    fn new(_owner: &RigidBody) -> Self {
        BugTest {}
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<BugTest>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
