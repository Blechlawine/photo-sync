use std::path::PathBuf;

use gtk::traits::{
    BoxExt, ButtonExt, DialogExt, EntryExt, FileChooserExt, LabelExt, OrientableExt,
    ToggleButtonExt, WidgetExt,
};
use gtk::Orientation::Vertical;
use gtk::{FileChooserDialog, Inhibit, EditableSignals};

use relm::Widget;
use relm_derive::{widget, Msg};

pub struct Model {
    source_path: Option<PathBuf>,
    destination_path: Option<PathBuf>,
    group_by_creation_date: bool,
    date_format: Option<String>,
    recursive: bool,
}

#[derive(Msg)]
pub enum Msg {
    SourcePathSelected(PathBuf),
    DestinationPathSelected(PathBuf),
    UpdateGroupByCreationDate(bool),
    UpdateDateFormat(String),
    UpdateRecursive(bool),
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
            group_by_creation_date: true,
            date_format: None,
            recursive: true,
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
            Msg::UpdateGroupByCreationDate(group_by_creation_date) => {
                self.model.group_by_creation_date = group_by_creation_date;
            }
            Msg::UpdateDateFormat(date_format) => {
                self.model.date_format = Some(date_format);
            }
            Msg::UpdateRecursive(recursive) => {
                self.model.recursive = recursive;
            }
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                margin_top: 10,
                margin_bottom: 10,
                margin_start: 10,
                margin_end: 10,
                spacing: 10,
                gtk::Button {
                    clicked => Msg::OpenSourcePicker,
                    label: "Select source directory",
                },
                gtk::Label {
                    label: &self.model.source_path.as_ref().unwrap_or(&PathBuf::new()).to_string_lossy(),
                },
                gtk::CheckButton {
                    label: "Recursive",
                    active: self.model.recursive,
                    toggled(btn) => Msg::UpdateRecursive(btn.is_active()),
                },
                gtk::CheckButton {
                    label: "Group by creation date",
                    active: self.model.group_by_creation_date,
                    toggled(btn) => Msg::UpdateGroupByCreationDate(btn.is_active()),
                },
                gtk::Entry {
                    text: &self.model.date_format.as_ref().unwrap_or(&String::from("%Y-%m-%d")),
                    sensitive: self.model.group_by_creation_date,
                    changed(entry) => Msg::UpdateDateFormat(entry.text().to_string()),
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
