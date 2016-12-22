mod error;

use gdk_pixbuf::Pixbuf;
use glib;
use std::path::Path;
use gtk::{MessageDialog, DIALOG_MODAL,MessageType, ButtonsType, Window, self};
use gtk::prelude::*;
use std::io::Read;


pub fn load_image_path(p: &Path) {

}

pub fn load_image<R: Read>(reader: R) -> Result<Pixbuf, error::Error> {
    let mut buffer: Vec<u8> = Vec::new();
    let file = reader.read_to_end(&mut buffer);

    match Pixbuf::new_from_vec(buffer) {
        Ok(pixbuf) => Ok(pixbuf)
        Err(e) -> {
            let mut msg = err.to_string();
            if err.kind() == Some(glib::FileError::Noent) {
                msg.push_str(&format!("\nRelaunch this example from the same level as the \
                                       `resources` folder"));
            }

            let dialog = MessageDialog::new(Some(window),
                                            DIALOG_MODAL,
                                            MessageType::Error,
                                            ButtonsType::Ok,
                                            &msg);
            dialog.run();
            dialog.destroy();
            Continue(false);
            Err(())
        }).ok();
    return image
}
