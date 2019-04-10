extern crate image;
extern crate imageproc;
use image::RgbaImage;
use graphics_buffer::RenderBuffer;
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
    pub fn create_background_image(&self, _lane_graph: &LaneGraph, _map_bbox: &MapBbox) {
        /*const EDGE_WIDTH: f64 = 3.0;
        let path = Path::new("bg.png");
        let mut img = RgbaImage::new(self.width as u32, self.height as u32);
        draw_filled_rect_mut(
            &mut img,
            imageproc::rect::Rect::at(0, 0).of_size(self.width as u32, self.height as u32),
            Color::GREENFOREST.as_rgba()
        );
        let mut edges = lane_graph.lanes().all_edges();
        while let Some(edge) = edges.next() {
            let node1 = lane_graph.intersection(edge.0);
            let node2 = lane_graph.intersection(edge.1);
            let pos_node1: (f64, f64) = point_to_window(node1.position(), &self, &map_bbox);
            let pos_node2: (f64, f64) = point_to_window(node2.position(), &self, &map_bbox);
            draw_convex_polygon_mut(
                &mut img,
                &[Point::new(50, 60), Point::new(52, 58), Point::new(120, 103), Point::new(122, 101)],
                Color::LIGHTGRAY.as_rgba(),
            );
        }
        img.save(path).unwrap();*/
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
    _color: Color,
    _img: &mut RgbaImage,
) {
    let _rectangle_length: f64 = (p2.0 - p1.0).hypot(p2.1 - p1.1);
    let _rectangle_width: f64 = width;
    let _rectangle_angle: f64 = (p2.1 - p1.1).atan2(p2.0 - p1.0);
    /*let transform = IDENTITY
        .trans(p1.0, p1.1)
        .rot_rad(rectangle_angle)
        .scale(rectangle_length, rectangle_width);*/
    /*let points:&[Point<i32>] = &[
        Point::new(rectangle_length.0, p1.1),
        Point::new(p1.0 + EDGE_WIDTH, p1.1),
        Point::new(p2.0 + EDGE_WIDTH)
    ];*/
}
