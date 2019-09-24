use cairo::Context;
use gtk::{Inhibit, WidgetExt};
use serde_derive::{Deserialize, Serialize};
use std::cell::Cell;
use std::rc::Rc;
use glm::{Vec2,min};
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    pub back_color: [f64; 3],
    pub line_color: [f64; 3],
    pub full_size: [f64; 2],
    pub field_size: [f64; 2],
    pub goal_size: [f64; 2],
    pub goal_length: f64,
    pub center_diameter: f64,
    pub line_width: f64,
}

impl Default for Settings {
    fn default() -> Settings {
        //division Aを参考にした。
        Settings {
            back_color: [0.1, 0.9, 0.1],
            line_color: [0.9, 0.9, 0.9],
            full_size: [13400.0, 10400.0],
            field_size: [12000.0, 9000.0],
            goal_size: [1200.0, 2400.0],
            goal_length: 1200.0,
            center_diameter: 1000.0,
            line_width: 10.0,
        }
    }
}

pub struct FieldDrawing {
    settings: Settings,
    drawing_area: gtk::DrawingArea,
    is_drawing_stage: Cell<bool>,
}

impl FieldDrawing {
    pub fn new(settings: &Settings, drawing_area: gtk::DrawingArea) -> Rc<FieldDrawing> {
        let field = Rc::new(FieldDrawing {
            settings: settings.clone(),
            drawing_area: drawing_area,
            is_drawing_stage: Cell::new(true),
        });
        //assign event
        let field_drawing = field.clone();
        field
            .drawing_area
            .connect_draw(move |widget, cairo| field_drawing.draw(widget, cairo));
        field
    }

    fn draw(&self, widget: &gtk::DrawingArea, context: &Context) -> Inhibit {
        let settings=&self.settings;
        //scaling
        let (pixel_x, pixel_y) = (
            widget.get_allocated_width() as f64,
            widget.get_allocated_height() as f64,
        );
        let [full_x, full_y] = settings.full_size;
        let scale = min(pixel_x / full_x, pixel_y / full_y); //適切な変換係数を求める
        context.translate(pixel_x / 2.0, pixel_y / 2.0);
        context.scale(scale, -scale);
        //drawing
        context.save();
        self.draw_clear(context);
        if self.is_drawing_stage.get() {}
        context.restore();
        Inhibit(false)
    }

    fn draw_clear(&self,context:&Context){
        let settings = &self.settings;
        let [back_red, back_green, back_blue] = settings.back_color;
        let [full_x,full_y]=settings.full_size;
        context.set_source_rgb(back_red, back_green, back_blue);
        context.rectangle(-full_x/2.0,-full_y/2.0,full_x/2.0,full_y/2.0);
        context.fill();
    }

    fn draw_stage(&self,context:&Context){
        let settings=&self.settings;
        let [field_x, field_y] = settings.field_size;
        context.rectangle(-field_x / 2.0, -field_y / 2.0, field_x, field_y);
        context.stroke();
        //center cicle
        let center_diameter = settings.center_diameter;
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
