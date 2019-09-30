use super::model;
use crate::listener;
use cairo::Context;
use glm::{distance, min, Vec2};
use gtk::{Inhibit, WidgetExt};
use serde_derive::{Deserialize, Serialize};
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::f64::consts::PI;
use std::rc::Rc;
use std::time;
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    pub back_color: [f64; 3],
    pub line_color: [f64; 3],
    pub ball_color: [f64; 3],
    pub yellow_color: [f64; 3],
    pub blue_color: [f64; 3],
    pub full_size: [f64; 2],
    pub field_size: [f64; 2],
    pub goal_size: [f64; 2],
    pub goal_length: f64,
    pub center_diameter: f64,
    pub line_width: f64,
    pub ball_diameter: f64,
    pub robot_diameter: f64,
    pub gain_default: f64, //拡大率(対象はロボットとボール、ライン)
}

impl Default for Settings {
    fn default() -> Settings {
        //division Aを参考にした。
        Settings {
            back_color: [0.05, 0.2, 0.05],
            line_color: [0.95, 0.95, 0.95],
            ball_color: [1.0, 0.7, 0.0],
            yellow_color: [0.8, 0.6, 0.3],
            blue_color: [0.2, 0.2, 0.95],
            full_size: [13400.0, 10400.0],
            field_size: [12000.0, 9000.0],
            goal_size: [1200.0, 2400.0],
            goal_length: 1200.0,
            center_diameter: 1000.0,
            line_width: 10.0,
            ball_diameter: 43.0,
            robot_diameter: 170.0,
            gain_default: 10.0,
        }
    }
}

#[derive(Debug)]
pub struct Flags {
    pub gain: Cell<f64>,
    pub is_drawing_stage: Cell<bool>,
    pub is_drawing_balls: Cell<bool>,
    pub is_drawing_robots: Cell<bool>,
}

impl Default for Flags {
    fn default() -> Flags {
        Flags {
            gain: Cell::new(10.0),
            is_drawing_stage: Cell::new(true),
            is_drawing_balls: Cell::new(true),
            is_drawing_robots: Cell::new(true),
        }
    }
}

pub struct FieldDrawing {
    settings: Settings,
    drawing_area: gtk::DrawingArea,
    pub flags: Flags,
    pub items: RefCell<model::Items>,
    fps_last: Cell<time::Instant>,
}

fn set_color(context: &Context, rgb: &[f64; 3]) {
    let [r, g, b] = rgb;
    context.set_source_rgb(*r, *g, *b);
}

impl FieldDrawing {
    pub fn new(settings: &Settings, drawing_area: gtk::DrawingArea) -> Rc<FieldDrawing> {
        let field = Rc::new(FieldDrawing {
            settings: settings.clone(),
            drawing_area: drawing_area,
            flags: Flags::default(),
            items: RefCell::new(model::Items::default()),
            fps_last: Cell::new(time::Instant::now()),
        });
        //assign event
        let field_drawing = field.clone();
        field
            .drawing_area
            .connect_draw(move |widget, cairo| field_drawing.draw(widget, cairo));

        field
    }

    pub fn update(&self, w: &listener::World) {
        self.items.borrow_mut().update(w);
        self.drawing_area.queue_draw();
    }

    fn draw(&self, _widget: &gtk::DrawingArea, context: &Context) -> Inhibit {
        //drawing
        self.draw_clear(context);
        if self.flags.is_drawing_stage.get() {
            self.draw_stage(context)
        }
        if self.flags.is_drawing_balls.get() {
            self.draw_balls(context)
        }
        if self.flags.is_drawing_robots.get() {
            self.draw_robots(context)
        }
        //draw fps
        context.set_font_size(24.0);
        context.move_to(10.0, 100.0);
        let fps_now = time::Instant::now();
        let diff = fps_now - self.fps_last.get();
        context.show_text(&format!(
            "{:?},items={}",
            diff,
            self.items.borrow().blues.len()
        ));
        //self.fps_last.set(fps_now);
        Inhibit(false)
    }

    fn pixel_size(&self) -> (f64, f64) {
        (
            self.drawing_area.get_allocated_width() as f64,
            self.drawing_area.get_allocated_height() as f64,
        )
    }

    fn transform_real(&self, context: &Context) {
        let settings = &self.settings;
        //scaling
        let (pixel_x, pixel_y) = self.pixel_size();
        let [full_x, full_y] = settings.full_size;
        let scale = min(pixel_x / full_x, pixel_y / full_y); //適切な変換係数を求める
        context.translate(pixel_x / 2.0, pixel_y / 2.0);
        context.scale(scale, -scale);
    }

    fn draw_clear(&self, context: &Context) {
        let settings = &self.settings;
        let [back_red, back_green, back_blue] = settings.back_color;
        let (pixel_x, pixel_y) = self.pixel_size();
        context.save();
        context.set_source_rgb(back_red, back_green, back_blue);
        context.rectangle(0.0, 0.0, pixel_x, pixel_y);
        context.fill();
        context.restore();
    }

    fn draw_stage(&self, context: &Context) {
        let settings = &self.settings;
        let [field_x, field_y] = settings.field_size;
        let [line_red, line_green, line_blue] = settings.line_color;
        context.save();
        self.transform_real(context);
        context.set_line_width(settings.line_width * self.flags.gain.get());
        context.set_source_rgb(line_red, line_green, line_blue);
        context.rectangle(-field_x / 2.0, -field_y / 2.0, field_x, field_y);
        context.stroke();
        //center cicle
        let center_diameter = settings.center_diameter;
        context.arc(0.0, 0.0, center_diameter / 2.0, 0.0, 2.0 * PI);
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
        context.restore();
    }

    fn draw_balls(&self, context: &Context) {
        let radius = self.flags.gain.get() * self.settings.ball_diameter / 2.0;
        context.save();
        self.transform_real(context);
        set_color(context, &self.settings.ball_color);
        for ball in self.items.borrow().balls.iter() {
            let (x, y) = (ball.position.x as f64, ball.position.y as f64);
            context.arc(x, y, radius, 0.0, 2.0 * PI);
            context.fill();
        }
        context.restore();
    }

    fn draw_robots(&self, context: &Context) {
        context.save();
        self.transform_real(context);
        for blue in self.items.borrow().blues.iter(){
            context.move_to(blue.position.x as f64, blue.position.y as f64);
            set_color(context,&self.settings.blue_color);
            context.arc(blue.position.x as f64, blue.position.y as f64,self.settings.robot_diameter/2.0, blue.angle as f64+ PI/6.0,blue.angle as f64-PI/6.0);
            context.fill();
            //println!("{:?}",blue);
        } 


        context.restore();
    }
}
