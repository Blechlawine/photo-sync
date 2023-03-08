use gtk::{glib, Application, Box, Button, Label, Orientation};
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

    let btn_open_source_picker = Button::builder()
        .label("Select source directory")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let lbl_source_path = Label::builder()
        .label("No source directory selected")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let box_container = Box::new(Orientation::Vertical, 0);
    box_container.append(&btn_open_source_picker);
    box_container.append(&lbl_source_path);

    file_chooser.add_button("Select", gtk::ResponseType::Accept);

    file_chooser.connect_response(move |dialog, response| {
        if response == gtk::ResponseType::Accept {
            if let Some(file) = dialog.file() {
                lbl_source_path.set_label(&file.path().unwrap().to_string_lossy());
                dialog.destroy();
            }
        }
    });

    btn_open_source_picker.connect_clicked(move |_| {
        file_chooser.show();
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Photosync")
        .child(&box_container)
        .build();

    window.present();
}
