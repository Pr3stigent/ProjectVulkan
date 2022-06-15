use once_cell::sync::Lazy;
use std::hash::Hash;
use std::sync::Mutex;
use std::sync::Arc;

use winit::{
    event::{ Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState }
};

#[allow(dead_code)]
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum InputEvent {
    Began(fn(Input)),
    Changed(fn(Input)),
    Ended(fn(Input))
}


#[allow(dead_code)]
pub enum Hold {
    OneKey,
    AllKeys,
}

#[allow(dead_code)]
pub struct InputStruct {
    pub event: InputEvent,
    keys: Option<VirtualKeyCode>,
    state: bool,
}

#[derive(Debug)]
pub struct Input {
    pub key_code: VirtualKeyCode
}

pub static CALLBACKS: Lazy<Arc<Mutex<Vec<InputStruct>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));


#[allow(dead_code)]
fn callback_handler(virtual_keycode: Option<VirtualKeyCode>, input_event: &mut InputStruct, callback: fn(Input)) {
    match virtual_keycode {
        Some(virtual_keycode) => {
            if let Some(_) = input_event.keys {
                if virtual_keycode != input_event.keys.unwrap() {
                    return
                }
            }
            
            callback(Input {
                key_code: virtual_keycode
            });
            input_event.state = true;
        },
        None => {}
    }
}

pub fn on_input(event: InputEvent, keys: &Option<VirtualKeyCode>) {
    CALLBACKS
        .lock()
        .unwrap()
        .push(InputStruct {
            event: event,
            keys: *keys,
            state: false,
        });
}

#[allow(dead_code)]
pub fn process_event(event:  &Event<()>) {
    match event {
        Event::WindowEvent { event, .. } => match *event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode,
                    ..
                },
                ..
            } => {
                match state {
                    ElementState::Pressed => {
                        for input_event in CALLBACKS.lock().unwrap().iter_mut() {
                            match input_event.event {
                                InputEvent::Began(callback) => {
                                    if state == ElementState::Pressed && !input_event.state {
                                        callback_handler(virtual_keycode, input_event, callback);
                                    }
                                },
                                /*InputEvent::Changed(callback) => {

                                },*/
                                InputEvent::Ended(_) => {
                                    if state == ElementState::Pressed && input_event.state {
                                        input_event.state = false;
                                    }
                                }
                                _ => {}
                            }
                        }
                   },
                    ElementState::Released => {
                        for input_event in CALLBACKS.lock().unwrap().iter_mut() {
                            match input_event.event {
                                InputEvent::Began(_) => input_event.state = false,
                                /*InputEvent::Changed(callback) => {

                                },*/
                                InputEvent::Ended(callback) => callback_handler(virtual_keycode, input_event, callback),
                                _ => {}
                            }
                        }
                    } 
                }
                
            }
            _ => {}
        },
        _ => {}
    }
}
