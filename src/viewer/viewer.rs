//TODO *を直す
use crate::viewer::field::Field;
use cairo::Context;
use gio::prelude::*;
use glm::*;
use gtk::prelude::*;
use gtk::{DrawingArea, Window};
use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    pub height: i32,
    pub width: i32,
    pub field: Field,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            height: 480,
            width: 640,
            field: Field::default(),
        }
    }
}

pub struct Viewer {
    main_window: gtk::Window,
    field_drawing: gtk::DrawingArea,
    field_settings: Field,
}

impl Viewer {
    pub fn new(settings: &Settings) -> Rc<Viewer> {
        let ui_src = include_str!("viewer.ui");
        let ui_builder = gtk::Builder::new_from_string(ui_src);
        //load components
        let main_window: gtk::Window = ui_builder.get_object("MainWindow").unwrap();
        main_window.set_default_size(settings.width, settings.height);
        let field_drawing: gtk::DrawingArea = ui_builder.get_object("FieldDrawing").unwrap();
        //create instance
        let viewer = Rc::new(Viewer {
            main_window: main_window,
            field_drawing: field_drawing,
            field_settings: settings.field.clone(),
        });
        //assign event
        viewer.main_window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        {
            //drawing
            let clone = viewer.clone();
            viewer
                .field_drawing
                .connect_draw(move |widget, cairo| clone.draw_field(widget, cairo));
        }
        viewer.main_window.show_all();
        viewer
    }

    fn draw_field(&self, widget: &gtk::DrawingArea, context: &Context) -> Inhibit {
        let settings = self.field_settings;
        //省略名
        let (wx, wy) = (
            widget.get_allocated_width() as f64,
            widget.get_allocated_height() as f64,
        );
        let [fx, fy] = settings.full_size;
        let bc = settings.back_color;
        let lc = settings.line_color;
        let scale = min(wx / fx, wy / fy); //適切な変換係数[pixel/mm]を求める
                                           //draw begin
        context.save();
        //clear
        context.set_source_rgb(bc[0], bc[1], bc[2]);
        context.rectangle(0.0, 0.0, wx, wy);
        context.fill();
        //draw lines
        /*context.set_line_width(3.0);
        context.set_source_rgb(lc[0], lc[1], lc[2]);
        
        context.stroke();
        */
        context.set_source_rgb(lc[0], lc[1], lc[2]);
        context.translate(wx/2.0, wy/2.0);
        context.arc(0.0, 0.0,100.0 , 0.0,2.0*std::f64::consts::PI);
        //context.rectangle(10.0, 10.0, 100.0, 100.0);
        context.fill();
        //context.move_to(scale*700,scale );
        //draw end
        context.restore();
        Inhibit(false)
    }
}
