use opengl_graphics::GlGraphics;
use graphics::Context;
use graphics::rectangle;
use graphics::Transformed;
use crate::systems::renderer::color::Color;

pub trait Drawable {
    fn draw(&self, x: f64, y: f64, color: Color, c: Context, gl: &mut GlGraphics);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DrawableShape {
    Rectangle(Rectangle),
    Circle(Circle)
}

impl Drawable for DrawableShape {
    fn draw(&self, x: f64, y: f64, color: Color, c: Context, gl: &mut GlGraphics) {
        match self {
            DrawableShape::Rectangle(rectangle) => rectangle.draw(x, y, color, c, gl),
            DrawableShape::Circle(circle) => circle.draw(x, y, color, c, gl)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, x: f64, y: f64, color: Color, c: Context, gl: &mut GlGraphics) {
        let transform = c
            .transform
            .trans(x, y)
            .scale(self.width, self.height);
        rectangle(color.get(), rectangle::square(0.0, 0.0, 1.0), transform, gl);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Circle {
    width: f64,
    height: f64
}

impl Drawable for Circle {
    fn draw(&self, x: f64, y: f64, color: Color, c: Context, gl: &mut GlGraphics) {
        /*const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square = rectangle::square(0.0, 0.0, 1.0);
        let transform = c
            .transform
            .trans(pos.x.value_unsafe * 10., pos.y.value_unsafe * 10.)
            .scale(self.width, self.height);

        rectangle(RED, square, transform, gl);*/
    }
}