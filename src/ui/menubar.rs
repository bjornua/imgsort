use gtk;
use gtk::prelude::*;

pub struct MenuBar<'a, F: Fn(PathBuf) + 'a> {
    widget: gtk::MenuBar,
    parent: &'a gtk::Window,
    on_directory: &'a F
}

impl<'a, F: Fn(PathBuf) + 'a> MenuBar<'a, F> {
    pub fn new(parent: &'a gtk::Window, on_directory: &'a F) -> MenuBar<'a, F>  {
        let menubar = gtk::MenuBar::new();
        let menu_file = {
            let menu = gtk::Menu::new();

            let item = menu_item("_Directory", |_| ());
            item.set_submenu(Some(&menu));
            let parent = parent.clone();
            let item_open = menu_item("_Open Directory", move |_| {
                if let Some(path) = prompt_directory(&parent) {
                    on_directory(path)
                }
            });
            menu.add(&item_open);

            item
        };
        menubar.append(&menu_file);
        return MenuBar { parent: parent, widget: menubar, on_directory: on_directory}
    }
    pub fn set_directory() {

    }
    pub fn get_menubar<'c>(&'c self) -> &'c gtk::MenuBar {
        &self.widget
    }
}
use std::path::PathBuf;

pub fn prompt_directory(parent: &gtk::Window) -> Option<PathBuf> {
    let response_ok: i32 = gtk::ResponseType::Ok.into();
    let response_cancel: i32 = gtk::ResponseType::Cancel.into();
    let response_delete_event: i32 = gtk::ResponseType::DeleteEvent.into();

    let dialog = gtk::FileChooserDialog::new(None, Some(parent), gtk::FileChooserAction::SelectFolder);
    dialog.set_local_only(true);
    dialog.set_select_multiple(false);

    dialog.add_button("_Cancel", response_cancel);
    dialog.add_button("_Open Directory", response_ok);
    dialog.set_default_response(response_ok);

    let retval = match dialog.run() {
        n if n == response_ok => {
            let filename = dialog.get_filename();
            dialog.destroy();
            filename
        }
        n if n == response_cancel => {
            dialog.destroy();
            None
        }
        n if n == response_delete_event => {
            None
        }
        n => panic!("Unexpected response type in prompt_directory {}", n)
    };
    // dialog.destroy();
    retval
}

pub fn menu_item<F: Fn(&gtk::MenuItem) + 'static>(label: &'static str, action: F) -> gtk::MenuItem {
    let menuitem = gtk::MenuItem::new_with_mnemonic(label);
    menuitem.connect_activate(action);
    menuitem
}


// pub fn menu_item_img(icon_name: &'static str, label: &'static str) -> gtk::MenuItem {
//     let b = gtk::Box::new(gtk::Orientation::Horizontal, 6);
//     b.set_homogeneous(false);
//     b.set_halign(gtk::Align::Start);
//     let label = gtk::Label::new_with_mnemonic(Some(label));
//     let image = gtk::Image::new_from_icon_name(icon_name, gtk::IconSize::Menu.into());
//     b.set_hexpand(true);
//     b.add(&image);
//     b.pack_end(&label, true, true, 0);
//     let menuitem = gtk::MenuItem::new();
//     menuitem.add(&b);
//     menuitem
// }
