use crate::components::constant::Drawer;
use crate::components::statics::trafficlight::Light;
use crate::components::types::constant::CarType;
use crate::components::Position;
use crate::configurations::debugger::VisualDebugger;
use crate::ressources::generals::MapBbox;
use crate::ressources::lane_graph::LaneData;
use crate::ressources::lane_graph::LaneGraph;
use crate::systems::renderer::color::Color;
use crate::systems::renderer::drawableshape::Drawable;
use graphics::{clear, rectangle, Context, Transformed};
use opengl_graphics::GlGraphics;
use petgraph::graphmap::Neighbors;
use piston::input::RenderArgs;
use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect};

const EDGE_WIDTH: f64 = 2.0;

pub struct DrawClear;

impl<'a> System<'a> for DrawClear {
    type SystemData = (WriteExpect<'a, GlGraphics>, ReadExpect<'a, RenderArgs>);

    fn run(&mut self, (mut g_handle, args): Self::SystemData) {
        g_handle.draw(args.viewport(), |_, gl| {
            clear(Color::GREENFOREST.get(), gl);
        });
    }
}

pub struct DrawMap;

impl<'a> System<'a> for DrawMap {
    type SystemData = (
        ReadExpect<'a, VisualDebugger>,
        ReadExpect<'a, MapBbox>,
        ReadExpect<'a, LaneGraph>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(&mut self, (debugger, map_bbox, lane_graph, mut g_handle, args): Self::SystemData) {
        //polar_coordinates_to_cartesian
        //TODO:: replace this algorithm to simply interate between all edges
        for (nodeid, node) in lane_graph.intersections() {
            let pos_node: (f64, f64) = point_to_window(node.position(), &debugger, &map_bbox);

            let neighbors: Neighbors<'_, u64, petgraph::Directed> =
                lane_graph.graph.neighbors(*nodeid);

            for neighbor in neighbors {
                let _lane: &LaneData = lane_graph.lane_between((*nodeid, neighbor));

                let pos_neighbor: (f64, f64) = point_to_window(
                    lane_graph.intersection(neighbor).position(),
                    &debugger,
                    &map_bbox,
                );

                debug!(
                    "lane rendering: x1={} y1={} x2={} y2={}",
                    pos_node.0, pos_node.1, pos_neighbor.0, pos_neighbor.1
                );
                g_handle.draw(args.viewport(), |c, gl| {
                    draw_lane_between_two_points(
                        pos_node,
                        pos_neighbor,
                        EDGE_WIDTH,
                        Color::LIGHTGRAY,
                        c,
                        gl,
                    );
                });
            }
        }
    }
}

pub struct DrawTrafficLights;

impl<'a> System<'a> for DrawTrafficLights {
    type SystemData = (
        ReadExpect<'a, VisualDebugger>,
        ReadExpect<'a, MapBbox>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Light>,
        ReadStorage<'a, Drawer>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(
        &mut self,
        (debugger, map_bbox, positions, lights, drawers, mut g_handle, args): Self::SystemData,
    ) {
        for (position, light, drawer) in (&positions, &lights, &drawers).join() {
            let (x, y): (f64, f64) = pos_to_window(position, &debugger, &map_bbox);

            debug!("light rendering: x={} y={}", x, y);
            g_handle.draw(args.viewport(), |c, gl| {
                drawer
                    .figure
                    .draw(x, y, light.color.get_rendering_color(), c, gl);
            });
        }
    }
}

pub struct DrawVehicles;

impl<'a> System<'a> for DrawVehicles {
    type SystemData = (
        ReadExpect<'a, VisualDebugger>,
        ReadExpect<'a, MapBbox>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, CarType>,
        ReadStorage<'a, Drawer>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(
        &mut self,
        (debugger, map_bbox, positions, cars, drawers, mut g_handle, args): Self::SystemData,
    ) {
        for (position, _car, drawer) in (&positions, &cars, &drawers).join() {
            let (x, y): (f64, f64) = pos_to_window(position, &debugger, &map_bbox);

            debug!("vehicule2 rendering: x={} y={}", x, y);

            g_handle.draw(args.viewport(), |c, gl| {
                drawer.figure.draw(x, y, Color::BLACK, c, gl);
            });
        }
    }
}

fn draw_lane_between_two_points(
    p1: (f64, f64),
    p2: (f64, f64),
    width: f64,
    color: Color,
    c: Context,
    gl: &mut GlGraphics,
) {
    let rectangle_length: f64 = (p2.0 - p1.0).hypot(p2.1 - p1.1);
    let rectangle_width: f64 = width;
    let rectangle_angle: f64 = (p2.1 - p1.1).atan2(p2.0 - p1.0);

    let transform = c
        .transform
        .trans(p1.0, p1.1)
        .rot_rad(rectangle_angle)
        .scale(rectangle_length, rectangle_width);
    rectangle(color.get(), rectangle::square(0.0, 0.0, 1.0), transform, gl);
}

fn pos_to_window(pos: &Position, debugger: &VisualDebugger, map_bbox: &MapBbox) -> (f64, f64) {
    point_to_window(
        (pos.val.x.value_unsafe, pos.val.y.value_unsafe),
        debugger,
        map_bbox,
    )
}

fn point_to_window(
    (x, y): (f64, f64),
    debugger: &VisualDebugger,
    map_bbox: &MapBbox,
) -> (f64, f64) {
    let diff_x: f64 = map_bbox.x2 - map_bbox.x1;
    let diff_y: f64 = map_bbox.y2 - map_bbox.y1;
    let width: f64 = debugger.width;
    let height: f64 = debugger.height;
    let xpx = width * (x - map_bbox.x1) / diff_x;
    let ypx = height * (map_bbox.y2 - y) / diff_y;
    (xpx, ypx)
}
