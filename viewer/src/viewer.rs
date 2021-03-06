//TODO *を直す
use super::field;
use super::size_mode::SizeMode;
use gtk;
use gtk::prelude::*;
use model::World;
use serde_derive::{Deserialize, Serialize};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
//use crate::listener::World;
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
#[allow(dead_code)]
pub struct Viewer {
    main_window: gtk::Window,
    size_mode: Cell<SizeMode>,
    field_drawing: Rc<field::FieldDrawing>,
    world: Arc<RwLock<World>>,
}

impl Viewer {
    pub fn new(settings: &Settings, world: Arc<RwLock<World>>) -> Result<Rc<Viewer>, String> {
        if gtk::init().is_err() {
            return Err("gtk cannot initialize".to_owned());
        }
        let ui_src = include_str!("viewer.ui");
        let ui_builder = gtk::Builder::new_from_string(ui_src);
        //load components
        let main_window: gtk::Window = ui_builder
            .get_object("MainWindow").ok_or("Error:MainWindow is lost".to_owned())?;
        main_window.set_default_size(settings.width, settings.height);
        //
        let field_drawing_area: gtk::DrawingArea = ui_builder
            .get_object("FieldDrawing")
            .ok_or("Error:FieldDrawing is lost".to_owned())?;

        let field_draw =
            field::FieldDrawing::new(&settings.field, field_drawing_area, world.clone());
        //create instance
        let viewer = Rc::new(Viewer {
            main_window: main_window,
            size_mode: Cell::new(SizeMode::default()),
            field_drawing: field_draw,
            world: world,
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
        Ok(viewer)
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

    pub fn run(&self) {
        gtk::main();
    }
}
