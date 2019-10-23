use super::fps_counter::FPSCounter;
use cairo::Context;
use glib::source::Continue;
use glm::{cos, min, sin};
use gtk::{Inhibit, WidgetExt};
use model::{Ball, Robot, World};
use serde_derive::{Deserialize, Serialize};
use std::cell::{Cell, RefCell};
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use glib::object::Cast;
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    pub back_color: [f64; 3],
    pub line_color: [f64; 3],
    pub ball_color: [f64; 3],
    pub yellow_color: [f64; 3],
    pub blue_color: [f64; 3],
    pub front_color: [f64; 3],
    pub font_color: [f64; 3],
    pub hint_color: [f64; 3],
    pub full_size: [f64; 2],
    pub field_size: [f64; 2],
    pub goal_size: [f64; 2],
    pub goal_length: f64,
    pub center_diameter: f64,
    pub line_width: f64,
    pub ball_diameter: f64,
    pub robot_diameter: f64,
    pub gain_default: f64, //拡大率(対象はロボットとボール、ライン)
    pub hint_width: f64,
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
            front_color: [0.3, 0.0, 0.0],
            font_color: [0.9, 0.9, 0.9],
            hint_color: [0.1, 0.1, 0.1],
            full_size: [13400.0, 10400.0],
            field_size: [12000.0, 9000.0],
            goal_size: [1200.0, 2400.0],
            goal_length: 1200.0,
            center_diameter: 1000.0,
            line_width: 10.0,
            ball_diameter: 43.0,
            robot_diameter: 170.0,
            gain_default: 2.0,
            hint_width: 10.0,
        }
    }
}

#[derive(Debug)]
pub struct Flags {
    pub gain: Cell<f64>,
    pub is_drawing_stage: Cell<bool>,
    pub is_drawing_balls: Cell<bool>,
    pub is_drawing_robots: Cell<bool>,
    pub is_drawing_paths: Cell<bool>,
    pub blue_is_right: Cell<bool>,
}

impl Default for Flags {
    fn default() -> Flags {
        Flags {
            gain: Cell::new(1.0),
            is_drawing_stage: Cell::new(true),
            is_drawing_balls: Cell::new(true),
            is_drawing_robots: Cell::new(true),
            is_drawing_paths: Cell::new(true),
            blue_is_right: Cell::new(true),
        }
    }
}

pub struct FieldDrawing {
    settings: Settings,
    drawing_area: gtk::DrawingArea,
    pub flags: Flags,
    pub items: RefCell<model::World>,
    fps: FPSCounter,
    world: Arc<RwLock<World>>,
}

fn set_color(context: &Context, rgb: &[f64; 3]) {
    let [r, g, b] = rgb;
    context.set_source_rgb(*r, *g, *b);
}

impl FieldDrawing {
    pub fn new(
        settings: &Settings,
        world: Arc<RwLock<World>>,
    ) -> Rc<FieldDrawing> {

        let drawing_area=gtk::DrawingAreaBuilder::new().visible(true).build();

        let flags = Flags {
            gain: Cell::new(settings.gain_default),
            ..Flags::default()
        };
        let field = Rc::new(FieldDrawing {
            settings: settings.clone(),
            drawing_area: drawing_area,
            flags: flags,
            items: RefCell::new(model::World::default()),
            fps: FPSCounter::new(),
            world: world,
        });
        //assign event
        let field_drawing = field.clone();
        field
            .drawing_area
            .connect_draw(move |widget, cairo| field_drawing.draw(widget, cairo));

        let interval_ms = 1000 / 60;
        let field_timer = field.clone();
        gtk::timeout_add(interval_ms, move || {
            field_timer.drawing_area.queue_draw(); //wait for vsync
            Continue(true)
        });
        field
    }

    pub fn widget(&self)->&gtk::Widget{
        self.drawing_area.upcast_ref()
    }

    fn draw(&self, _widget: &gtk::DrawingArea, context: &Context) -> Inhibit {
        //clone
        let world = self.world.read().unwrap();
        //drawing
        self.draw_clear(context);
        if self.flags.is_drawing_stage.get() {
            self.draw_stage(context)
        }
        if self.flags.is_drawing_paths.get() {
            self.draw_paths(
                context,
                &world.balls,
                &world.blues.robots,
                &world.yellows.robots,
            );
        }
        if self.flags.is_drawing_balls.get() {
            self.draw_balls(context, &world.balls);
        }
        if self.flags.is_drawing_robots.get() {
            self.draw_robots(context, &world.blues.robots, &world.yellows.robots);
        }

        //draw fps
        context.save();
        context.set_font_size(24.0);
        context.move_to(10.0, 100.0);
        context.show_text(&format!("fps={}", self.fps.count()));
        context.restore();

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
        context.identity_matrix();
        context.translate(pixel_x / 2.0, pixel_y / 2.0);
        context.scale(scale, scale);
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

    fn draw_balls(&self, context: &Context, balls: &Vec<Box<Ball>>) {
        let radius = self.flags.gain.get() * self.settings.ball_diameter / 2.0;
        context.save();
        self.transform_real(context);
        set_color(context, &self.settings.ball_color);
        for ball in balls.iter() {
            let (x, y) = (ball.position.x as f64, ball.position.y as f64);
            context.arc(x, y, radius, 0.0, 2.0 * PI);
            context.fill();
        }
        context.restore();
    }

    fn draw_robots(&self, context: &Context, blues: &Vec<Box<Robot>>, yellows: &Vec<Box<Robot>>) {
        let radius = self.flags.gain.get() * self.settings.robot_diameter / 2.0;
        context.save();
        self.transform_real(context);

        let draw = |robots: &Vec<Box<Robot>>, color: &[f64; 3]| {
            for robot in robots.iter() {
                let (x, y, rad) = (
                    robot.position.x as f64,
                    -robot.position.y as f64,
                    robot.angle as f64,
                );
                let (rad_begin, rad_end) = (rad + PI / 6.0, rad - PI / 6.0);
                //筐体
                context.move_to(x, y);
                set_color(context, color);
                context.arc(x, y, radius, 0.0, PI * 2.0);
                context.fill();
                context.set_line_width(radius * 0.2);
                //向き
                context.move_to(x + radius * cos(rad_end), y + radius * sin(rad_end));
                set_color(context, &self.settings.front_color);
                context.arc(x, y, radius, rad_end, rad_begin);
                context.stroke();
                //ID
                context.move_to(x, y);
                context.set_line_width(radius / 10.0);
                context.set_font_size(radius);
                set_color(context, &self.settings.font_color);
                let tex_id = context.text_extents(&format!("{}", robot.id));
                context.move_to(x - tex_id.width / 2.0, y + tex_id.height / 2.0);
                context.show_text(&format!("{}", robot.id));
                context.stroke();
                //context.rotate(-PI);
            }
        };
        draw(blues, &self.settings.blue_color);
        draw(yellows, &self.settings.yellow_color);
        context.restore();
    }

    fn draw_paths(
        &self,
        context: &Context,
        balls: &Vec<Box<Ball>>,
        blues: &Vec<Box<Robot>>,
        yellows: &Vec<Box<Robot>>,
    ) {
        context.save();
        self.transform_real(context);
        let draw = |items: Vec<(_, _)>| {
            //下三角要素を抽出する
            for (index, from) in items.iter().enumerate() {
                for to in items[0..index].iter() {
                    context.move_to(from.0 as f64, from.1 as f64);
                    context.line_to(to.0 as f64, to.1 as f64);
                    context.stroke();
                }
            }
        };
        context.set_line_width(self.settings.hint_width * self.settings.gain_default);
        set_color(context, &self.settings.hint_color);
        //TODO わかりにくいので命名を変えるべし
        let right_goal = ((self.settings.field_size[0] / 2.0) as f32, 0 as f32);
        let left_goal = ((-self.settings.field_size[0] / 2.0) as f32, 0 as f32);
        let (blue_goal, yellow_goal) = if self.flags.blue_is_right.get() {
            (right_goal, left_goal)
        } else {
            (left_goal, right_goal)
        };

        let mut blue_positions: Vec<(_, _)> = blues
            .iter()
            .map(|v| (v.position.x, -v.position.y))
            .chain(balls.iter().map(|v| (v.position.x, v.position.y)))
            .collect();
        blue_positions.push(blue_goal);
        let mut yellow_positions: Vec<(_, _)> = yellows
            .iter()
            .map(|v| (v.position.x, -v.position.y))
            .chain(balls.iter().map(|v| (v.position.x, v.position.y)))
            .collect();
        yellow_positions.push(yellow_goal);

        draw(blue_positions);
        draw(yellow_positions);
        context.restore();
    }
}
