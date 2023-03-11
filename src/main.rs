use std::path::PathBuf;

use chrono::{DateTime, Utc};
use fs_extra::file::CopyOptions;
use gtk::traits::{
    BoxExt, ButtonExt, DialogExt, EntryExt, FileChooserExt, LabelExt, OrientableExt,
    ToggleButtonExt, WidgetExt,
};
use gtk::Orientation::Vertical;
use gtk::{EditableSignals, FileChooserDialog, Inhibit};

use relm::Widget;
use relm_derive::{widget, Msg};

pub struct Model {
    source_path: Option<PathBuf>,
    destination_path: Option<PathBuf>,
    group_by_creation_date: bool,
    copy: bool, // true for copying, false for moving
    date_format: String,
    recursive: bool,
}

#[derive(Msg)]
pub enum Msg {
    SourcePathSelected(PathBuf),
    DestinationPathSelected(PathBuf),
    UpdateGroupByCreationDate(bool),
    UpdateDateFormat(String),
    UpdateRecursive(bool),
    UpdateCopy(bool),
    MoveFiles,
    OpenSourcePicker,
    OpenDestinationPicker,
    Quit,
}

fn get_all_files(start_path: &PathBuf, recursive: bool) -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in std::fs::read_dir(start_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if recursive {
                files.append(&mut get_all_files(&path, recursive));
            }
        } else if path.is_file() {
            files.push(path);
        }
    }
    files
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

    fn move_files(&self) {
        // if recursive is true, step through source path and move all files including subfolders
        let source_path = self.model.source_path.as_ref().unwrap();
        let destination_path = self.model.destination_path.as_ref().unwrap();
        let mut files = get_all_files(source_path, self.model.recursive);
        for file in files.iter_mut() {
            let destination_file = {
                if self.model.group_by_creation_date {
                    // read creation date of file
                    let metadata = std::fs::metadata(&file).unwrap();
                    let creation_date = metadata.created().unwrap();
                    let date_format = self.model.date_format.as_ref();
                    let creation_date_utc: DateTime<Utc> = DateTime::from(creation_date);

                    let result = destination_path.join(PathBuf::from(
                        creation_date_utc.format(date_format).to_string(),
                    ));
                    if !result.exists() {
                        std::fs::create_dir_all(&result).unwrap();
                    }
                    result.join(file.file_name().unwrap())
                } else {
                    destination_path.join(file.file_name().unwrap())
                }
            };
            if self.model.copy {
                match std::fs::copy(&file, &destination_file) {
                    Ok(_) => {
                        println!(
                            "File {} copied to {}",
                            file.display(),
                            destination_file.display()
                        );
                    }
                    Err(e) => {
                        println!(
                            "Error copying file {} to {}: {}",
                            file.display(),
                            destination_file.display(),
                            e
                        );
                    }
                }
            } else {
                match fs_extra::file::move_file(&file, &destination_file, &CopyOptions::new()) {
                    Ok(_) => {
                        println!(
                            "File {} moved to {}",
                            file.display(),
                            destination_file.display()
                        );
                    }
                    Err(e) => {
                        println!(
                            "Failed to move {} to {}: {}",
                            file.display(),
                            destination_file.display(),
                            e
                        );
                    }
                };
            }
        }
    }
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            source_path: None,
            destination_path: None,
            group_by_creation_date: true,
            date_format: "%Y-%m-%d".to_string(),
            recursive: true,
            copy: true,
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
                self.model.date_format = date_format;
            }
            Msg::UpdateRecursive(recursive) => {
                self.model.recursive = recursive;
            }
            Msg::UpdateCopy(copy) => {
                self.model.copy = copy;
            }
            Msg::MoveFiles => {
                self.move_files();
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
                    label: "Copy files instead of moving",
                    active: self.model.copy,
                    toggled(btn) => Msg::UpdateCopy(btn.is_active()),
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
                    text: &self.model.date_format.as_ref(),
                    sensitive: self.model.group_by_creation_date,
                    changed(entry) => Msg::UpdateDateFormat(entry.text().to_string()),
                },
                gtk::Button {
                    clicked => Msg::OpenDestinationPicker,
                    label: "Select destination directory",
                },
                gtk::Label {
                    label: &self.model.destination_path.as_ref().unwrap_or(&PathBuf::new()).to_string_lossy(),
                },
                gtk::Button {
                    label: "Move files",
                    sensitive: self.model.source_path.is_some() && self.model.destination_path.is_some(),
                    clicked => Msg::MoveFiles,
                }
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
