use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use specs::{ReadExpect, ReadStorage, System, Join, WriteExpect};
use graphics::*;
use crate::components::constant::Drawer;
use crate::components::dynamic::Position;
use crate::systems::renderer::Color;
use crate::systems::renderer::drawableshape::Drawable;

pub struct DrawClear;
impl<'a> System<'a> for DrawClear {
    type SystemData = (WriteExpect<'a, GlGraphics>, ReadExpect<'a, RenderArgs>);

    fn run(&mut self, (mut g_handle, args): Self::SystemData) {
        g_handle.draw(args.viewport(), |_, gl| {
            clear(Color::LIGHTGRAY.get(), gl);
        });
    }
}

pub struct DrawRectangles;
impl<'a> System<'a> for DrawRectangles {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Drawer>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(&mut self, (positions, drawers, mut g_handle, args): Self::SystemData) {
        /*let laneGraph: LaneGraph = LaneGraph::new(
            [
                (1u64, IntersectionData::new(10.0, 10.0)),
                (2u64, IntersectionData::new(10.0, 24.0)),
                (3u64, IntersectionData::new(30.0, 17.0)),
                (4u64, IntersectionData::new(40.0, 17.0)),
            ]
            .to_vec()
            .into_iter(),
            &[
                (1, 3, LaneData::new(None, None, None)),
                (2, 3, LaneData::new(None, None, None)),
                (3, 4, LaneData::new(None, None, None)),
            ],
        );*/
        for (position, drawer) in (&positions, &drawers).join() {
            g_handle.draw(args.viewport(), |c, gl| {
                drawer.figure.draw(position.x.value_unsafe * 10., position.y.value_unsafe * 10., Color::RED, c, gl);
            });
        }
    }
}