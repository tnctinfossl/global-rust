//TODO *を直す
use crate::viewer::field;
use crate::viewer::size_mode::SizeMode;
use gtk::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::cell::{Cell,RefMut,Ref};
use std::rc::Rc;
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    pub height: i32,
    pub width: i32,
    pub field: field::Settings,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            height: 480,
            width: 640,
            field: field::Settings::default(),
        }
    }
}

pub struct Viewer {
    main_window: gtk::Window,
    size_mode: Cell<SizeMode>,
    field_drawing:Rc<field::FieldDrawing>
}

impl Viewer {
    pub fn new(settings: &Settings) -> Rc<Viewer> {
        let ui_src = include_str!("viewer.ui");
        let ui_builder = gtk::Builder::new_from_string(ui_src);
        //load components
        let main_window: gtk::Window = ui_builder.get_object("MainWindow").expect("Error:MainWindow is lost");
        main_window.set_default_size(settings.width, settings.height);
        //
        let field_drawing_area: gtk::DrawingArea = ui_builder.get_object("FieldDrawing").expect("Error:FieldDrawing is lost");
        let field_draw=field::FieldDrawing::new(&settings.field,field_drawing_area);
        //create instance
        let viewer = Rc::new(Viewer {
            main_window: main_window,
            size_mode: Cell::new(SizeMode::default()),
            field_drawing: field_draw
        });
        //assign event
        viewer.main_window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        {
            //key press
            let clone = viewer.clone();
            viewer
                .main_window
                .connect_key_press_event(move |winwdow, key| clone.press_key(winwdow, key));
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
    
    pub fn items_borrow(&self)-> Ref<field::Items>{
        self.field_drawing.items_borrow()
    }

    pub fn items_borrow_mut(&self)-> RefMut<field::Items>{
        self.field_drawing.items_borrow_mut()
    }
}
