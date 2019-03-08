use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use specs::{ReadExpect, ReadStorage, System, Join, WriteExpect};
use graphics::*;
use crate::components::constant::Drawer;
use crate::components::dynamic::Position;
use crate::ressources::lane_graph::IntersectionData;
use crate::ressources::lane_graph::LaneData;
use crate::ressources::lane_graph::LaneGraph;
use crate::systems::renderer::Color;
use crate::systems::renderer::drawableshape::Drawable;
use petgraph::graphmap::Neighbors;
use dim::si::{M};

pub struct DrawClear;
impl<'a> System<'a> for DrawClear {
    type SystemData = (WriteExpect<'a, GlGraphics>, ReadExpect<'a, RenderArgs>);

    fn run(&mut self, (mut g_handle, args): Self::SystemData) {
        g_handle.draw(args.viewport(), |_, gl| {
            clear(Color::LIGHTGRAY.get(), gl);
        });
    }
}

pub struct DrawMap;
impl<'a> System<'a> for DrawMap {
    type SystemData = (
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(&mut self, (mut g_handle, args): Self::SystemData) {
        let laneGraph: LaneGraph = LaneGraph::new(
            [
                (1u64, IntersectionData::new(10.0, 10.0)),
                (2u64, IntersectionData::new(10.0, 24.0)),
                (3u64, IntersectionData::new(30.0, 17.0)),
                (4u64, IntersectionData::new(40.0, 17.0)),
            ]
            .to_vec()
            .into_iter(),
            &[
                (1, 3, LaneData::new(Some(3.0 * M), None, None)),
                (2, 3, LaneData::new(Some(2.5 * M), None, None)),
                (3, 4, LaneData::new(Some(3.5 * M), None, None)),
            ],
        );
        //println!("{:#?}", laneGraph.get_nodes());
        for (nodeid, node) in laneGraph.get_nodes() {
            g_handle.draw(args.viewport(), |c, gl| {
                let transform = c
                    .transform
                    .trans(node.position.0 * 10., node.position.1 * 10.)
                    .scale(5.0, 5.0);
                rectangle(Color::BLACK.get(), rectangle::square(0.0, 0.0, 1.0), transform, gl);
                //drawer.figure.draw(position.x.value_unsafe * 10., position.y.value_unsafe * 10., Color::RED, c, gl);
            });
            let mut voisins: Neighbors<'_, u64, petgraph::Directed> = laneGraph.graph.neighbors(*nodeid);
            while let Some(voisin) = voisins.next() {
                let lane: &LaneData = laneGraph.lane_between((*nodeid, voisin));
                println!("{:#?}, {:#?}: {:#?}", nodeid, voisin, lane.width);
            }
        }
    }
}

pub struct DrawVehicles;
impl<'a> System<'a> for DrawVehicles {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Drawer>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(&mut self, (positions, drawers, mut g_handle, args): Self::SystemData) {
        for (position, drawer) in (&positions, &drawers).join() {
            g_handle.draw(args.viewport(), |c, gl| {
                drawer.figure.draw(position.x.value_unsafe * 10., position.y.value_unsafe * 10., Color::RED, c, gl);
            });
        }
    }
}