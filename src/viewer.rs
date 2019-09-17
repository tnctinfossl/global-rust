use gtk::{Application, ApplicationWindow, Button};
use gtk::prelude::*;
use gio::prelude::*;
use super::settings;

pub struct Viewer{
    
}


impl Viewer{
    pub fn new(settings:&settings::Viewer)->Viewer{
        Viewer{

        }
    }
}





//TODO あとでクラス化する
pub fn window_run(){
    let application = Application::new(Some("com.github.tnctinfossl.global-rust"), Default::default())
        .expect("failed to initialize GTK application");
    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        window.add(&button);

        window.show_all();
    });


    application.run(&[]);
}

