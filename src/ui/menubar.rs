use gtk;
use gtk::prelude::*;

pub struct MenuBar {
    widget: gtk::MenuBar,
    parent: gtk::Window
}

impl MenuBar {
    pub fn new(parent: &gtk::Window) -> Self  {
        let parent = parent.clone();

        let menubar = gtk::MenuBar::new();
        let menu_file = {
            let menu = gtk::Menu::new();

            let item = menu_item("_Directory", |_| ());
            item.set_submenu(Some(&menu));
            let parent = parent.clone();
            let item_open = menu_item("_Open Directory", move |_| {
                gtk::FileChooserDialog::new(None, Some(&parent), gtk::FileChooserAction::SelectFolder).run();
            });
            menu.add(&item_open);

            item
        };
        menubar.append(&menu_file);
        return MenuBar { parent: parent, widget: menubar }
    }
    pub fn get_menubar<'b>(&'b self) -> &'b gtk::MenuBar {
        &self.widget
    }
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
