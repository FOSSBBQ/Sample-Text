extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Sample Text");
    window.set_default_size(100, 100);
    let open_dir = Button::new_with_label("Open Project Directory");
    let open_file = Button::new_with_label("Open File");
    window.add(&open_dir);
    window.add(&open_file);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    open_dir.connect_clicked(|_| {
        println!("Clicked!");
    });

    open_file.connect_clicked(|_| {
        println!("Opening file...")
    });

    gtk::main();
}
