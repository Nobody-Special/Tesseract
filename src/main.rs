extern crate gtk;
extern crate glib;
extern crate gio;
extern crate gdk;
extern crate gdk_pixbuf;

use gtk::prelude::*;
use gio::{ActionMapExt, ApplicationExt, MenuExt, SimpleActionExt};

#[macro_use] extern crate log;
extern crate loggerv;

use log::LogLevel;

fn main() {
	if gtk::init().is_err() { println!("Failed to initialize GTK.") }

	let window = gtk::Window::new(gtk::WindowType::Toplevel);

	window.set_title("First GTK+ Program");
	window.set_border_width(10);
	window.set_position(gtk::WindowPosition::Center);
	window.set_default_size(350, 70);

	window.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
	});

	window.show_all();
	gtk::main();
}
