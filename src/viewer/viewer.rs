//TODO *を直す
use crate::viewer::field::Field;
use crate::viewer::size_mode::SizeMode;
use cairo::Context;
use gio::prelude::*;
use glm::*;
use gtk::prelude::*;
use gtk::{DrawingArea, Window};
use serde_derive::{Deserialize, Serialize};
use std::cell::Cell;
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
    size_mode: Cell<SizeMode>,
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
            size_mode: Cell::new(SizeMode::default()),
            field_drawing: field_drawing,
            field_settings: settings.field.clone(),
        });
        //assign event
        viewer.main_window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        {
            //key press
            let mut clone = viewer.clone();
            viewer
                .main_window
                .connect_key_press_event(move |winwdow, key| clone.press_key(winwdow, key));
        }

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

    fn press_key(&self, window: &gtk::Window, key: &gdk::EventKey) -> Inhibit {
        match key.get_hardware_keycode() {
            //F1 size down
            67 => {
                if let Some(back) = self.size_mode.get().back() {
                    let size = back.size();
                    window.resize(size.0, size.1);
                    self.size_mode.set(back);
                }
            }
            //F2 size reset
            68 => {
                let default = SizeMode::default();
                if self.size_mode.get() != default {
                    let size = default.size();
                    window.resize(size.0, size.1);
                    self.size_mode.set(default);
                }
            }
            //F3 size up
            69 => {
                if let Some(next) = self.size_mode.get().next() {
                    let size = next.size();
                    window.resize(size.0, size.1);
                    self.size_mode.set(next);
                }
            }
            _ => (),
        }

        //println!("group={},key={}",key.get_group(),key.get_hardware_keycode());
        Inhibit(true)
    }
    


    fn draw_field(&self, widget: &gtk::DrawingArea, context: &Context) -> Inhibit {
        let settings = self.field_settings;
        //省略名
        let (pixel_x, pixel_y) = (
            widget.get_allocated_width() as f64,
            widget.get_allocated_height() as f64,
        );
        let [full_x, full_y] = settings.full_size;
        let scale = min(pixel_x / full_x, pixel_y / full_y); //適切な変換係数を求める
        let gain = 10.0; //大きさの拡大率(表示用)
        context.save();
        //clear
        let [back_red, back_green, back_blue] = settings.back_color;
        context.set_source_rgb(back_red, back_green, back_blue);
        context.rectangle(0.0, 0.0, pixel_x, pixel_y);
        context.fill();
        //scaling for 
        let [line_red, line_green, line_blue] = settings.line_color;
        context.set_source_rgb(line_red, line_green, line_blue);
        context.translate(pixel_x / 2.0, pixel_y / 2.0);
        context.scale(scale, -scale);
        //draw field
        context.set_line_width(gain * 10.0);
        Viewer::draw_stage(context, &settings);
        //draw end
        context.restore();
        println!("{}", scale);
        Inhibit(false)
    }

    fn draw_stage(context:&Context,settings:&Field){
        //draw field rectangle
        let [field_x, field_y] = settings.field_size;
        let center_diameter = settings.center_diameter;
        context.rectangle(-field_x / 2.0, -field_y / 2.0, field_x, field_y);
        context.stroke();
        //center cicle
        context.arc(
            0.0,
            0.0,
            center_diameter / 2.0,
            0.0,
            2.0 * std::f64::consts::PI,
        );
        context.stroke();
        //center line
        context.move_to(0.0, -field_y / 2.0);
        context.line_to(0.0, field_y / 2.0);
        context.move_to(-field_x / 2.0, 0.0);
        context.line_to(field_x / 2.0, 0.0);
        context.stroke();
        //draw left goal
        let [goal_x, goal_y] = settings.goal_size;
        context.move_to(-field_x / 2.0, goal_y / 2.0);
        context.rel_line_to(goal_x, 0.0);
        context.rel_line_to(0.0, -goal_y);
        context.rel_line_to(-goal_x, 0.0);
        context.stroke();
        //draw right goal
        context.move_to(field_x / 2.0, goal_y / 2.0);
        context.rel_line_to(-goal_x, 0.0);
        context.rel_line_to(0.0, -goal_y);
        context.rel_line_to(goal_x, 0.0);
        context.stroke();
    }
}
