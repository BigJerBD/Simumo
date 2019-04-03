use specs::World;
use graphics::*;
use graphics_buffer::*;
use crate::ressources::lane_graph::LaneGraph;

///Represent the ending time of the simulator.
#[derive(Clone, Deserialize)]
pub struct VisualDebugger {
    #[serde(rename = "use")]
    pub on: bool,
    pub width: f64,
    pub height: f64,
    #[serde(skip_deserializing)]
    #[serde(default = "default_resource")]
    pub bgimage: RenderBuffer,
}

fn default_resource() -> RenderBuffer {
    RenderBuffer::new(0, 0)
}

impl VisualDebugger {
    pub fn create_background_image(&self, lane_graph: &LaneGraph) {
        const EDGE_WIDTH: f64 = 3.0;
        let width: u32 = self.width as u32;
        let height: u32 = self.height as u32;
        let color_bg = [0.2, 0.53, 0.12, 1.0];
        let street_bg = [0.75, 0.75, 0.75, 1.0];
        let mut buffer = RenderBuffer::new(width, height);
        buffer.clear(color_bg);

        /*for (nodeid, node) in lane_graph.intersections() {
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
                g_handle.draw(args.viewport(), |c, gl| {
                    draw_lane_between_two_points(
                        pos_node,
                        pos_neighbor,
                        EDGE_WIDTH,
                        street_bg,
                        c,
                        gl,
                    );
                });
            }
        }*/
        /*let p1: (f64, f64) = (3.0, 20.0);
        let p2: (f64, f64) = (120.0, 400.0);
        let width = 3.0;
        let rectangle_length: f64 = (p2.0 - p1.0).hypot(p2.1 - p1.1);
        let rectangle_width: f64 = width;
        let rectangle_angle: f64 = (p2.1 - p1.1).atan2(p2.0 - p1.0);
        let transform = IDENTITY
            .trans(p1.0, p1.1)
            .rot_rad(rectangle_angle)
            .scale(rectangle_length, rectangle_width);
        rectangle(
            street_bg,
            rectangle::square(0.0, 0.0, 1.0),
            transform,
            &mut buffer,
        );*/

        // Save the buffer
        buffer.save("background.png").unwrap();
    }
}