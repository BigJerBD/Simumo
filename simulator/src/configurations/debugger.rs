use graphics::*;
use graphics_buffer::*;
use crate::ressources::generals::MapBbox;
use crate::ressources::lane_graph::LaneGraph;
use crate::systems::renderer::color::Color;

///Represent the ending time of the simulator.
#[derive(Clone, Deserialize)]
pub struct VisualDebugger {
    #[serde(rename = "use")]
    pub on: bool,
    pub width: f64,
    pub height: f64,
    #[serde(skip_deserializing)]
    #[serde(default = "default_renderbuffer")]
    pub bgimage: RenderBuffer,
}

fn default_renderbuffer() -> RenderBuffer {
    RenderBuffer::new(0, 0)
}

impl VisualDebugger {
    pub fn create_background_image(&self, lane_graph: &LaneGraph, map_bbox: &MapBbox) {
        /*const EDGE_WIDTH: f64 = 3.0;
        let width: u32 = self.width as u32;
        let height: u32 = self.height as u32;
        let mut buffer = RenderBuffer::new(width, height);
        buffer.clear(Color::GREENFOREST.get());
        let mut edges = lane_graph.lanes().all_edges();
        while let Some(edge) = edges.next() {
            let node1 = lane_graph.intersection(edge.0);
            let node2 = lane_graph.intersection(edge.1);
            let pos_node1: (f64, f64) = point_to_window(node1.position(), &self, &map_bbox);
            let pos_node2: (f64, f64) = point_to_window(node2.position(), &self, &map_bbox);
            let p1 = pos_node1;
            let p2 = pos_node2;
            let rectangle_length: f64 = (p2.0 - p1.0).hypot(p2.1 - p1.1);
            let rectangle_width: f64 = EDGE_WIDTH;
            let rectangle_angle: f64 = (p2.1 - p1.1).atan2(p2.0 - p1.0);
            let transform = IDENTITY
                .trans(p1.0, p1.1)
                .rot_rad(rectangle_angle)
                .scale(rectangle_length, rectangle_width);
            rectangle(
                Color::LIGHTGRAY.get(),
                rectangle::square(0.0, 0.0, 1.0),
                transform,
                &mut buffer
            );
        }
        // Save the buffer
        buffer.save("background.png").unwrap();*/
    }
}

pub fn point_to_window(
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

fn draw_lane_between_two_points(
    p1: (f64, f64),
    p2: (f64, f64),
    width: f64,
    color: Color,
    buffer: &mut RenderBuffer,
) {
    let rectangle_length: f64 = (p2.0 - p1.0).hypot(p2.1 - p1.1);
    let rectangle_width: f64 = width;
    let rectangle_angle: f64 = (p2.1 - p1.1).atan2(p2.0 - p1.0);
    let transform = IDENTITY
        .trans(p1.0, p1.1)
        .rot_rad(rectangle_angle)
        .scale(rectangle_length, rectangle_width);
    /*rectangle(
        color.get(),
        rectangle::square(0.0, 0.0, 1.0),
        transform,
        &mut buffer
    );*/
}
