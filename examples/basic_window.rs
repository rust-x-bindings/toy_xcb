
extern crate toy_xcb;

use toy_xcb::{Window, Event};

fn main() {

    let w = Window::new(640, 480, "Example".to_string());

    'mainloop:
    loop {
        if let Some(ev) = w.wait_event() {
            match ev {
                Event::MousePress(pos, _, _) => {
                    println!("clicked window: {:?}", pos);
                },
                Event::Resize(size) => {
                    println!("resized window: {:?}", size);
                },
                Event::KeyPress(sym, code, text) => {
                    println!("key typed: sym={:?}, code={:?}, text=\"{}\"", sym, code, text);
                }
                Event::Close => {
                    println!("user close request");
                    break 'mainloop;
                },
                _ => {}
            }
        }
    }
}
