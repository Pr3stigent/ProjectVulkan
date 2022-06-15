mod application;

#[path="crates/geometry.rs"]
mod geometry;

use winit::event::VirtualKeyCode;
use crate::application::window_surface::input_controller::{ Input, InputEvent, on_input};

fn main() {
    on_input(InputEvent::Began(| input: Input | {
        println!("Started {:?} any", input.key_code)
    }), &None); // gets fired when a key has been pressed

    on_input(InputEvent::Began(| input: Input | {
        println!("Started {:?} just A", input.key_code)
    }), &Some(VirtualKeyCode::A)); // gets fired when the key "A" has been pressed

    
    let _ = application::init("A", [600, 600]);
}