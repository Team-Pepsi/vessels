use crate::graphics2_d::Vec2D;
use crate::input;
use crate::input::mouse;
use crate::input::mouse::{Action, Button, Event};

use stdweb::web::event::*;
use stdweb::web::{document, IEventTarget};

use std::cell::RefCell;
use std::rc::Rc;

pub struct MouseState {
    handlers: Vec<Box<Fn(input::mouse::Event)>>,
    position: Vec2D,
}

pub struct Mouse {
    state: Rc<RefCell<MouseState>>,
}

impl input::Source<mouse::Event> for Mouse {
    fn bind<F>(&self, handler: F)
    where
        F: Fn(input::mouse::Event) + 'static,
    {
        self.state.borrow_mut().handlers.push(Box::new(handler));
    }
}

impl input::Mouse for Mouse {
    fn position(&self) -> Vec2D {
        self.state.borrow().position
    }
}

impl Mouse {
    pub fn new() -> Mouse {
        let mouse = Mouse {
            state: Rc::new(RefCell::new(MouseState {
                handlers: vec![],
                position: Vec2D::default(),
            })),
        };
        mouse.initialize();
        mouse
    }
    fn initialize(&self) {
        let state = self.state.clone();
        let up_state = state.clone();
        let move_state = up_state.clone();
        let body = document().body().unwrap();
        body.add_event_listener(move |event: MouseDownEvent| {
            event.prevent_default();
            let state = state.borrow();
            state.handlers.iter().for_each(|handler| {
                handler(Event {
                    action: Action::Down(match event.button() {
                        MouseButton::Left => Button::Left,
                        MouseButton::Right => Button::Right,
                        MouseButton::Wheel => Button::Middle,
                        MouseButton::Button4 => Button::Auxiliary(0),
                        MouseButton::Button5 => Button::Auxiliary(1),
                    }),
                    position: state.position,
                })
            });
        });
        body.add_event_listener(move |event: MouseUpEvent| {
            event.prevent_default();
            let state = up_state.borrow();
            state.handlers.iter().for_each(|handler| {
                handler(Event {
                    action: Action::Up(match event.button() {
                        MouseButton::Left => Button::Left,
                        MouseButton::Right => Button::Right,
                        MouseButton::Wheel => Button::Middle,
                        MouseButton::Button4 => Button::Auxiliary(0),
                        MouseButton::Button5 => Button::Auxiliary(1),
                    }),
                    position: state.position,
                })
            });
        });
        body.add_event_listener(move |event: MouseMoveEvent| {
            let mut state = move_state.borrow_mut();
            event.prevent_default();
            state.position = (f64::from(event.client_x()), f64::from(event.client_y())).into();
            state.handlers.iter().for_each(|handler| {
                handler(Event {
                    action: Action::Move(
                        (f64::from(event.movement_x()), f64::from(event.movement_y())).into(),
                    ),
                    position: state.position,
                })
            })
        });
    }
}
