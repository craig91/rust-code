use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.HelloWorldl";

fn main() {
    // create a new application
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    // run the application
    app.run();
}

fn build_ui(app: &Application) {

    // create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // connect to "clicked" signal of `button`
    button.connect_clicked(move |button|  {
        // Set the label to "hello world!" after the button has been clicked on
        button.set_label("Hello World!");
    });
    //create a windows and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    window.present();
}