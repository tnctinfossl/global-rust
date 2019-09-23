use gio::prelude::*;
use gtk::prelude::*;
use gtk::*;
use serde_derive::{Deserialize, Serialize};
use std::io;

use cairo::{Context, RectangleInt};
#[derive(Copy, Clone, Serialize, Deserialize,Debug)]
pub struct Settings {
    pub height: i32,
    pub width: i32,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            height: 480,
            width: 640,
        }
    }
}


pub struct MainWindow {
    main_window: gtk::Window,
    field_drawing: gtk::DrawingArea,
}

impl MainWindow {
    pub fn new(settings:&Settings) -> MainWindow {
        let ui_src = include_str!("viewer.ui");
        let ui_builder = gtk::Builder::new_from_string(ui_src);
        let main_window: gtk::Window = ui_builder.get_object("MainWindow").unwrap();
        main_window.connect_delete_event(MainWindow::main_window_quit);
        main_window.set_default_size(settings.width,settings.height);
        let field_drawing: gtk::DrawingArea = ui_builder.get_object("FieldDrawing").unwrap();
        field_drawing.connect_draw(move |_widget, cairo_ctx| {
            cairo_ctx.save();
            cairo_ctx.move_to(50.0,100.0);
            cairo_ctx.set_font_size(18.0);
            cairo_ctx.show_text("The only curse they could afford to put on a tomb these days was 'Bugger Off'. --PTerry");
            cairo_ctx.restore();
            Inhibit(false)
        });
        main_window.show_all();
        MainWindow {
            main_window: main_window,
            field_drawing: field_drawing,
        }
    }

    fn main_window_quit(_:&gtk::Window, _: &gdk::Event)->Inhibit {
        gtk::main_quit();
        Inhibit(false)
    }

    //fn field_drawing_draw()
}
