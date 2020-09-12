use gdnative::api::RigidBody;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(RigidBody)]
pub struct BugTest {}

#[methods]
impl BugTest {
    #[export]
    fn _ready(&self, _owner: &RigidBody) {
        // Notice here _owner.add_central_force is not hinted
        // (or any of the methods available on RigidBody or other inherited traits)

        _owner.add_central_force(Vector3::new(0f32, 1f32, 0f32));

        godot_print!("hello Bug");
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
