use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use specs::{Component, ReadExpect, ReadStorage, System, VecStorage, WriteExpect};

use crate::components::constant::Rectangle;
use crate::components::dynamic::Position;

pub struct DrawClear;
impl<'a> System<'a> for DrawClear {
    type SystemData = (
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>);

    fn run(&mut self, (mut g_handle, args): Self::SystemData) {
        use graphics::*;
        const LIGHTGRAY: [f32; 4] = [0.85, 0.85, 0.85, 1.0];
        g_handle.draw(args.viewport(), |_, gl| {
            // Clear the screen
            clear(LIGHTGRAY, gl);
        });
    }
}

pub struct DrawRectangles;
impl<'a> System<'a> for DrawRectangles {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rectangle>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>);

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (pos_dat, rect_dat, mut g_handle, args) = data;

        for (pos, rect) in (&pos_dat, &rect_dat).join() {
            use graphics::*;

            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
            let square = rectangle::square(0.0, 0.0, 1.0);

            g_handle.draw(args.viewport(), |c, gl| {
                // TODO:: Au lieu d'utiliser value_unsafe, est-ce qu'on peut obtenir la valeur en Meters par exemple?
                // TODO:: Au lieu de multiplier par 10, utiliser une échelle appropriée compte tenu de la hauteur et largeur de la boite dans laquelle se déplacent les voitures
                let transform = c.transform
                    .trans(pos.x.value_unsafe * 10., pos.y.value_unsafe * 10.)
                    .scale(rect.width, rect.height);

                rectangle(RED, square, transform, gl);
            });
        }
    }
}