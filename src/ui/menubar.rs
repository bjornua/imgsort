use gtk;
use gtk::prelude::*;

pub struct MenuBar{
    widget: gtk::MenuBar,
    // parent: &'a gtk::Window,
}

impl MenuBar {
    pub fn new<F: Fn(Vec<PathBuf>) + 'static>(parent: &gtk::Window, on_files: F) -> MenuBar  {
        let menubar = gtk::MenuBar::new();
        let menu_file = {
            let menu = gtk::Menu::new();

            let item = menu_item("_File", |_| ());
            item.set_submenu(Some(&menu));
            let parent = parent.clone();
            let item_open = menu_item("_Add files", move |_| {
                on_files(prompt_files_add(&parent))
            });
            menu.add(&item_open);

            item
        };
        menubar.append(&menu_file);
        return MenuBar { widget: menubar }
    }
    // pub fn set_directory() {

    // }
    pub fn get_menubar<'c>(&'c self) -> &'c gtk::MenuBar {
        &self.widget
    }
}
use std::path::PathBuf;


pub fn menu_item<F: Fn(&gtk::MenuItem) + 'static>(label: &'static str, action: F) -> gtk::MenuItem {
    let menuitem = gtk::MenuItem::new_with_mnemonic(label);
    menuitem.connect_activate(action);
    menuitem
}


pub fn prompt_files_add(parent: &gtk::Window) -> Vec<PathBuf> {
    let response_ok: i32 = gtk::ResponseType::Ok.into();
    let response_cancel: i32 = gtk::ResponseType::Cancel.into();
    let response_delete_event: i32 = gtk::ResponseType::DeleteEvent.into();

    let dialog = gtk::FileChooserDialog::new(None, Some(parent), gtk::FileChooserAction::Open);
    dialog.set_local_only(true);
    dialog.set_select_multiple(true);

    dialog.add_button("_Cancel", response_cancel);
    dialog.add_button("_Add", response_ok);
    dialog.set_default_response(response_ok);

    let retval = match dialog.run() {
        n if n == response_ok => {
            let filename = dialog.get_filenames();
            dialog.destroy();
            filename
        }
        n if n == response_cancel => {
            dialog.destroy();
            vec![]
        }
        n if n == response_delete_event => {
            vec![]
        }
        n => panic!("Unexpected response type in prompt_directory {}", n)
    };
    retval
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
