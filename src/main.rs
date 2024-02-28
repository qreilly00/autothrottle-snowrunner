use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder, Fullscreen},
};

use enigo::*;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 1024.0))
        //.with_maximized(true)
        .build(&event_loop)
        .unwrap();

    let mut enigo = Enigo::new();
    let mut hold_state = false;

    event_loop.run(move |event, elwt| {
        match event {
            Event::AboutToWait => {
                // Application update code.
                hold_state = start_end_throttle(hold_state, &mut enigo);
            },

            _ => ()
        }

        if let Event::WindowEvent {event, ..} = event {
            match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                },

                WindowEvent::KeyboardInput {event: KeyEvent, ..} => {
                    //elwt.exit();
                },

                _ => ()
            }
        }
    });

}

fn start_end_throttle(hold_state: bool, enigo: &mut Enigo) -> bool {
    let mut tmp = false;
    if hold_state == false {
        enigo.key_down(Key::Layout('a'));
        tmp = true;
    } else {
        enigo.key_up(Key::Layout('a'));
        tmp = false;
    }

    tmp
}

fn listen_input() {

}

fn press_w() {

}

fn release_w() {

}
