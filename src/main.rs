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
    fn open_folder_picker(&self) -> Option<PathBuf> {
        let folder_dialog = FileChooserDialog::new::<gtk::Window>(
            Some("Select source directory"),
            Some(&self.root()),
            gtk::FileChooserAction::SelectFolder,
        );
        folder_dialog.set_select_multiple(false);
        folder_dialog.add_button("Select", gtk::ResponseType::Accept);
        if folder_dialog.run() == gtk::ResponseType::Accept {
            let path = folder_dialog.filename();
            folder_dialog.emit_close();
            return path;
        }
        folder_dialog.emit_close();
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
                self.model.source_path = self.open_folder_picker();
            }
            Msg::OpenDestinationPicker => {
                self.model.destination_path = self.open_folder_picker();
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
