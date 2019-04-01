use crate::configurations::generals::VisualDebugger;
use crate::components::constant::Drawer;
use crate::components::dynamic::Position;
use crate::components::dynamic::Speed;
use crate::components::statics::trafficlight::Light;
use crate::ressources::generals::MapBbox;
use crate::ressources::lane_graph::LaneData;
use crate::ressources::lane_graph::LaneGraph;
use crate::systems::renderer::drawableshape::Drawable;
use crate::systems::renderer::Color;
use crate::util::polar_coordinates_to_cartesian;
use graphics::{clear, rectangle, Context, Transformed};
use opengl_graphics::GlGraphics;
use petgraph::graphmap::Neighbors;
use piston::input::RenderArgs;
use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect};

const EDGE_WIDTH: f64 = 2.0;
const ZOOM_FACTOR: f64 = 2.0;

pub struct DrawClear;
impl<'a> System<'a> for DrawClear {
    type SystemData = (WriteExpect<'a, GlGraphics>, ReadExpect<'a, RenderArgs>);

    fn run(&mut self, (mut g_handle, args): Self::SystemData) {
        g_handle.draw(args.viewport(), |_, gl| {
            clear(Color::GREENGRASS.get(), gl);
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
        for (nodeid, node) in lane_graph.intersections() {
            let pos_node: (f64, f64) = polar_coordinates_to_window(
                node.position(),
                &debugger,
                &map_bbox,
            );
            
            let neighbors: Neighbors<'_, u64, petgraph::Directed> =
                lane_graph.graph.neighbors(*nodeid);

            for neighbor in neighbors {
                let _lane: &LaneData = lane_graph.lane_between((*nodeid, neighbor));
                let pos_neighbor: (f64, f64) = polar_coordinates_to_window(
                    lane_graph.intersection(neighbor).position(),
                    &debugger,
                    &map_bbox,
                );

                g_handle.draw(args.viewport(), |c, gl| {
                    draw_lane_between_two_points(
                        pos_node,
                        pos_neighbor,
                        EDGE_WIDTH,
                        Color::GRAY,
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
        ReadStorage<'a, Position>,
        ReadStorage<'a, Light>,
        ReadStorage<'a, Drawer>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(&mut self, (positions, lights, drawers, mut g_handle, args): Self::SystemData) {
        for (position, light, drawer) in (&positions, &lights, &drawers).join() {
            g_handle.draw(args.viewport(), |c, gl| {
                drawer.figure.draw(
                    position.x.value_unsafe * ZOOM_FACTOR,
                    position.y.value_unsafe * ZOOM_FACTOR,
                    light.color.get_rendering_color(),
                    c,
                    gl,
                );
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
        ReadStorage<'a, Speed>,
        ReadStorage<'a, Drawer>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(&mut self, (debugger, map_bbox, positions, speeds, drawers, mut g_handle, args): Self::SystemData) {
        for (position, _speed, drawer) in (&positions, &speeds, &drawers).join() {
            let pos_vehicle: (f64, f64) = cartesian_coordinates_to_window(
                (position.x.value_unsafe, position.y.value_unsafe),
                &debugger,
                &map_bbox,
            );
            g_handle.draw(args.viewport(), |c, gl| {
                drawer.figure.draw(
                    pos_vehicle.0,
                    pos_vehicle.1,
                    Color::BLACK,
                    c,
                    gl,
                );
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

fn polar_coordinates_to_window(polar_coord: (f64, f64), debugger: &VisualDebugger, map_bbox: &MapBbox) -> (f64, f64) {
    let (x, y) = polar_coordinates_to_cartesian(polar_coord);
    cartesian_coordinates_to_window(
        polar_coordinates_to_cartesian(polar_coord),
        debugger,
        map_bbox,
    )
}

fn cartesian_coordinates_to_window(cart_coord: (f64, f64), debugger: &VisualDebugger, map_bbox: &MapBbox) -> (f64, f64) {
    let (x, y) = cart_coord;
    let (min_x, min_y) = polar_coordinates_to_cartesian((map_bbox.lon1, map_bbox.lat1));
    let (max_x, max_y) = polar_coordinates_to_cartesian((map_bbox.lon2, map_bbox.lat2));
    let diff_x: f64 = max_x - min_x;
    let diff_y: f64 = max_y - min_y;
    let width: f64 = debugger.width;
    let height: f64 = debugger.height;
    let xpx = width * (x - min_x) / diff_x;
    let ypx = height * (max_y - y) / diff_y;
    (xpx, ypx)
}
