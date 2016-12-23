

use gtk;
use gtk::prelude::*;

pub fn init() -> gtk::MenuBar  {
    let menubar = gtk::MenuBar::new();

    let menu_file = {
        let menu = gtk::Menu::new();
        let item = menu_item("_File");
        item.set_submenu(Some(&menu));

        let item_open = menu_item_img("gtk-directory", "_Open Folder");
        menu.add(&item_open);

        item
    };

    menubar.append(&menu_file);

    return menubar
}


pub fn menu_item(label: &'static str) -> gtk::MenuItem {
    let menuitem = gtk::MenuItem::new_with_mnemonic(label);
    menuitem
}

pub fn menu_item_img(icon_name: &'static str, label: &'static str) -> gtk::MenuItem {
    let b = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    b.set_homogeneous(false);
    b.set_halign(gtk::Align::Start);
    let label = gtk::Label::new_with_mnemonic(Some(label));
    let image = gtk::Image::new_from_icon_name(icon_name, gtk::IconSize::Menu.into());
    b.set_hexpand(true);
    b.add(&image);
    b.pack_start(&label, true, true, 0);
    let menuitem = gtk::MenuItem::new();
    menuitem.add(&b);

    menuitem
}
