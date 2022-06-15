mod application;
mod input_controller;

use winit::event::VirtualKeyCode;
use input_controller::{ Input, InputEvent};

fn main() {
    input_controller::on_input(InputEvent::Began(| input: Input | {
        println!("Started {:?} any", input.key_code)
    }), &None); // gets fired when a key has been pressed

    input_controller::on_input(InputEvent::Began(| input: Input | {
        println!("Started {:?} just A", input.key_code)
    }), &Some(VirtualKeyCode::A)); // gets fired when the key "A" has been pressed

    
    let _ = application::init("A", [600, 600]);
}