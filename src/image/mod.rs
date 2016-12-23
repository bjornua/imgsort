mod error;

use gdk_pixbuf::Pixbuf;
use glib;
use gtk;
use std::path::Path;


pub fn from_path<P: AsRef<Path>>(path: P) -> Result<gtk::Image, error::Error> {
    let path = match path.as_ref().to_str() {
        Some(s) => s,
        None => return Err(error::Error::PathUTF8Error),
    };

    let pixbuf = match Pixbuf::new_from_file(path) {
        Ok(pixbuf) => pixbuf,
        Err(e) => {
            return match e.kind() {
                Some(glib::FileError::Noent) => Err(error::Error::FileNotFound(e)),
                Some(_) | None => return Err(error::Error::GLibError(e)),
            }
        }
    };
    let image = gtk::Image::new_from_pixbuf(Some(&pixbuf));
    Ok(image)
}
//

// if err.kind() == Some(glib::FileError::Noent) {
//     msg.push_str(&format!("\nRelaunch this example from the same level as the \
//                            `resources` folder"));
// }

// let dialog = MessageDialog::new(Some(window),
//                                 DIALOG_MODAL,
//                                 MessageType::Error,
//                                 ButtonsType::Ok,
//                                 &msg);
// dialog.run();
// dialog.destroy();
// Continue(false);
// Err(())
