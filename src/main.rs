use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        panic!("GTK+ initialization failed");
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_property_default_width(300);
    window.set_title("GTK+ Hello");
    window.set_resizable(false);
    let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let message = gtk::Label::new("Hello, world!");
    layout.add(&message);
    let close = gtk::Button::new_with_label("Close");
    layout.add(&close);
    window.add(&layout);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    close.connect_clicked(move |_| window.close());

    gtk::main();
}
