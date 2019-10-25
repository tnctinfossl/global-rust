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
use super::info_tree::InfoTree;
use super::widget::AsWidget;
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
    info_tree:Rc<InfoTree>,
    world: Arc<RwLock<World>>,
}

impl Viewer {
    pub fn new(settings: &Settings, world: Arc<RwLock<World>>) -> Result<Rc<Viewer>, String> {
        if gtk::init().is_err() {
            return Err("gtk cannot initialize".to_owned());
        }

        let field_drawing = field::FieldDrawing::new(&settings.field, world.clone());
        let main_window = gtk::WindowBuilder::new()
            .title("Global")
            .width_request(settings.width)
            .height_request(settings.height)
            .resizable(true)
            .visible(true)
            .build();
        main_window.add(field_drawing.as_widget_ref());

        //create instance
        let viewer = Rc::new(Viewer {
            main_window: main_window,
            size_mode: Cell::new(SizeMode::default()),
            field_drawing: field_drawing,
            world: world.clone(),
            info_tree:InfoTree::new(world.clone())
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
            //F4 path show
            70 => {
                let flag = &self.field_drawing.flags.is_drawing_paths;
                flag.set(!flag.get());
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
