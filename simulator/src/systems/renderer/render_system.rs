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
use graphics::*;
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
        for (nodeid, node) in lane_graph.intersections() {
            let pos_node: (f64, f64) = coordinates_to_window(
                node.position(),
                &debugger,
                &map_bbox,
            );
            
            let neighbors: Neighbors<'_, u64, petgraph::Directed> =
                lane_graph.graph.neighbors(*nodeid);

            for neighbor in neighbors {
                let _lane: &LaneData = lane_graph.lane_between((*nodeid, neighbor));
                let pos_neighbor: (f64, f64) = coordinates_to_window(
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
        ReadStorage<'a, Position>,
        ReadStorage<'a, Speed>,
        ReadStorage<'a, Drawer>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(&mut self, (positions, speeds, drawers, mut g_handle, args): Self::SystemData) {
        for (position, _speed, drawer) in (&positions, &speeds, &drawers).join() {
            g_handle.draw(args.viewport(), |c, gl| {
                drawer.figure.draw(
                    position.x.value_unsafe * ZOOM_FACTOR,
                    position.y.value_unsafe * ZOOM_FACTOR,
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

fn coordinates_to_window(coord: (f64, f64), debugger: &VisualDebugger, map_bbox: &MapBbox) -> (f64, f64) {
    let min_lon = map_bbox.x1;
    let max_lat = map_bbox.y2;
    let diff_lon: f64 = map_bbox.x2 - map_bbox.x1;
    let diff_lat: f64 = map_bbox.y2 - map_bbox.y1;
    let width: f64 = debugger.width;
    let height: f64 = debugger.height;
    let (lon, lat) = coord;
    let x = width * (lon - min_lon) / diff_lon;
    let y = height * (max_lat - lat) / diff_lat;
    (x, y)
}
