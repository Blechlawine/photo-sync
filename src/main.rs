use std::path::PathBuf;

use gtk::traits::{ButtonExt, DialogExt, FileChooserExt, LabelExt, OrientableExt, WidgetExt};
use gtk::Orientation::Vertical;
use gtk::{FileChooserDialog, Inhibit};

use relm::Widget;
use relm_derive::{widget, Msg};

pub struct Model {
    source_path: Option<PathBuf>,
    destination_path: Option<PathBuf>,
}

#[derive(Msg)]
pub enum Msg {
    SourcePathSelected(PathBuf),
    DestinationPathSelected(PathBuf),
    OpenSourcePicker,
    OpenDestinationPicker,
    Quit,
}

impl Win {
    fn open_source_picker(&self) -> Option<PathBuf> {
        let source_dialog = FileChooserDialog::new::<gtk::Window>(
            Some("Select source directory"),
            Some(&self.root()),
            gtk::FileChooserAction::SelectFolder,
        );
        source_dialog.set_select_multiple(false);
        source_dialog.add_button("Select", gtk::ResponseType::Accept);
        if source_dialog.run() == gtk::ResponseType::Accept {
            let path = source_dialog.filename();
            source_dialog.emit_close();
            return path;
        }
        source_dialog.emit_close();
        None
    }

    fn open_destination_picker(&self) -> Option<PathBuf> {
        let destination_dialog = FileChooserDialog::new::<gtk::Window>(
            Some("Select destination directory"),
            Some(&self.root()),
            gtk::FileChooserAction::SelectFolder,
        );
        destination_dialog.set_select_multiple(false);
        destination_dialog.add_button("Select", gtk::ResponseType::Accept);
        if destination_dialog.run() == gtk::ResponseType::Accept {
            let path = destination_dialog.filename();
            destination_dialog.emit_close();
            return path;
        }
        destination_dialog.emit_close();
        None
    }
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            source_path: None,
            destination_path: None,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::SourcePathSelected(path) => {
                self.model.source_path = Some(path);
            }
            Msg::DestinationPathSelected(path) => {
                self.model.destination_path = Some(path);
            }
            Msg::OpenSourcePicker => {
                self.model.source_path = self.open_source_picker();
            }
            Msg::OpenDestinationPicker => {
                self.model.destination_path = self.open_destination_picker();
            }
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                gtk::Button {
                    clicked => Msg::OpenSourcePicker,
                    label: "Select source directory",
                },
                gtk::Label {
                    label: &self.model.source_path.as_ref().unwrap_or(&PathBuf::new()).to_string_lossy(),
                },
                gtk::Button {
                    clicked => Msg::OpenDestinationPicker,
                    label: "Select destination directory",
                },
                gtk::Label {
                    label: &self.model.destination_path.as_ref().unwrap_or(&PathBuf::new()).to_string_lossy(),
                }
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
