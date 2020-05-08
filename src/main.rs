use {gio::prelude::*, glib::clone, gtk::prelude::*, std::env};

fn main() {
    let application = gtk::Application::new(
        Some("org.crazynest.gtk-hello"),
        gio::ApplicationFlags::empty(),
    )
    .expect("Failed to initialize GTK.");

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_property_default_width(300);
        window.set_title("Gtk-rs Hello");
        window.set_resizable(false);

        let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let message = gtk::Label::new(Some("Hello, world!"));
        layout.add(&message);

        let close = gtk::Button::new_with_label("Close");
        close.connect_clicked(clone!(@weak window => move |_| window.close()));
        layout.add(&close);

        window.add(&layout);

        window.show_all();
    });

    application.run(&env::args().collect::<Vec<_>>());
}
