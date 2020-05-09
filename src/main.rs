use {gio::prelude::*, glib::clone, gtk::prelude::*, std::env};

fn main() {
    if gtk::init().is_err() {
        panic!("Failed to initialize GTK.");
    }

    let application = gtk::ApplicationBuilder::new()
        .application_id("org.crazynest.gtk-hello")
        .build();

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindowBuilder::new()
            .application(app)
            .default_width(300)
            .resizable(false)
            .title("Gtk-rs Hello")
            .build();

        let layout = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let message = gtk::LabelBuilder::new().label("Hello, world!").build();
        layout.add(&message);

        let close = gtk::ButtonBuilder::new().label("Close").build();
        close.connect_clicked(clone!(@weak window => move |_| window.close()));
        layout.add(&close);

        window.add(&layout);

        window.show_all();
    });

    application.run(&env::args().collect::<Vec<_>>());
}
