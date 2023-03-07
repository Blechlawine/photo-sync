use gtk::{glib, Application, Button};
use gtk::{prelude::*, ApplicationWindow, FileChooserDialog};

const APP_ID: &str = "de.zmarc.photosync";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let file_chooser = FileChooserDialog::builder()
        .title("Select source directory")
        .action(gtk::FileChooserAction::SelectFolder)
        .build();

    file_chooser.add_button("Select", gtk::ResponseType::Accept);

    file_chooser.connect_response(move |dialog, response| {
        if response == gtk::ResponseType::Accept {
            if let Some(file) = dialog.file() {
                dbg!(file.path());
                dialog.destroy();
            }
        }
    });

    let btn_open_source_picker = Button::builder()
        .label("Select source directory")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    btn_open_source_picker.connect_clicked(move |_| {
        file_chooser.show();
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Photosync")
        .child(&btn_open_source_picker)
        .build();

    window.present();
}
