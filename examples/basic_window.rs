use toy_xcb::{Event, Result, Window};

fn main() -> Result<()> {
    let w = Window::new(640, 480, "Example".to_string())?;

    'mainloop: loop {
        match w.wait_event()? {
            Event::MousePress(pos, _, _) => {
                println!("clicked window: {:?}", pos);
            }
            Event::Resize(size) => {
                println!("resized window: {:?}", size);
            }
            Event::KeyPress(sym, code, text) => {
                println!(
                    "key typed: sym={:?}, code={:?}, text=\"{}\"",
                    sym, code, text
                );
            }
            Event::Close => {
                println!("user close request");
                break 'mainloop Ok(());
            }
            _ => {}
        }
    }
}
