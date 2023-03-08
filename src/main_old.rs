use gtk::{glib, Box, Button, Label, Orientation, Application};
use gtk::{prelude::*, ApplicationWindow, FileChooserDialog};

const APP_ID: &str = "de.zmarc.photosync";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let source_path_chooser = FileChooserDialog::builder()
        .title("Select source directory")
        .action(gtk::FileChooserAction::SelectFolder)
        .build();

    let destination_path_chooser = FileChooserDialog::builder()
        .title("Select destination directory")
        .action(gtk::FileChooserAction::SelectFolder)
        .build();

    let btn_open_source_picker = Button::builder().label("Select source directory").build();

    let lbl_source_path = Label::builder()
        .label("No source directory selected")
        .build();

    let btn_open_destination_picker = Button::builder()
        .label("Select destination directory")
        .build();

    let lbl_destination_path = Label::builder()
        .label("No destination directory selected")
        .build();

    let box_container = Box::new(Orientation::Vertical, 10);
    box_container.set_margin_bottom(10);
    box_container.set_margin_top(10);
    box_container.set_margin_start(10);
    box_container.set_margin_end(10);
    box_container.append(&btn_open_source_picker);
    box_container.append(&lbl_source_path);
    box_container.append(&btn_open_destination_picker);
    box_container.append(&lbl_destination_path);

    source_path_chooser.add_button("Select", gtk::ResponseType::Accept);

    source_path_chooser.connect_response(move |dialog, response| {
        if response == gtk::ResponseType::Accept {
            if let Some(file) = dialog.file() {
                lbl_source_path.set_label(&file.path().unwrap().to_string_lossy());
                dialog.destroy();
            }
        }
    });

    destination_path_chooser.add_button("Select", gtk::ResponseType::Accept);

    destination_path_chooser.connect_response(move |dialog, response| {
        if response == gtk::ResponseType::Accept {
            if let Some(file) = dialog.file() {
                lbl_destination_path.set_label(&file.path().unwrap().to_string_lossy());
                dialog.destroy();
            }
        }
    });

    btn_open_source_picker.connect_clicked(move |_| {
        source_path_chooser.show();
    });

    btn_open_destination_picker.connect_clicked(move |_| {
        destination_path_chooser.show();
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Photosync")
        .child(&box_container)
        .build();

    window.present();
}
