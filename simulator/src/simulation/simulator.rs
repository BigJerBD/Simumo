use dim::si::S;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::GlGraphics;
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use piston_window::OpenGL;
use piston_window::RenderEvent;
use specs::prelude::*;
use specs::Dispatcher;
use uuid::Uuid;

use crate::configurations::map;
use crate::configurations::Configuration;
use crate::entities::entity_type::Instantiable;
use crate::osmgraph_api::OsmGraphApi;
use crate::osmgraph_api::PythonOsmGraphApi;
use crate::ressources::clock;
use crate::ressources::eventsmanagement::EventsManager;
use crate::ressources::generals;
use crate::ressources::lane_graph::IntersectionData;
use crate::ressources::lane_graph::LaneData;
use crate::ressources::lane_graph::LaneGraph;
use crate::simulation::dispatchers::make_base_dispatcher;
use crate::simulation::dispatchers::make_render_dispatcher;

pub struct Simulation<'a, 'b> {
    world: World,
    window: Window,
    base_dispatcher: Dispatcher<'a, 'b>,
    render_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Simulation<'a, 'b> {
    const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

    pub fn from_config(config: Configuration) -> Self {
        //map Todo: uncommented below when query_graph works.
        // let _map:PythonOsmGraphApi = match config.map {
        //     map::Map::OsmGraph(val) => *(PythonOsmGraphApi::query_graph(val.longitude, val.latitude, val.zoom).unwrap()),
        // };

        //Todo: Systems

        let mut world = World::new();
        let window = Self::create_window();

        //ressources
        Self::create_resources(&mut world, config.generals.seed);

        let mut base_dispatcher = make_base_dispatcher();
        let mut render_dispatcher = make_render_dispatcher();
        base_dispatcher.setup(&mut world.res);
        render_dispatcher.setup(&mut world.res);

        //entities
        for entity in config.entities.iter() {
            entity.create(&mut world);
        }

        Self {
            world,
            window,
            base_dispatcher,
            render_dispatcher,
        }
    }

    pub fn run_simulation(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if !simulation_ended(&self.world) {
                self.base_dispatcher.dispatch(&self.world.res);
                self.world.maintain();
            }
            if let Some(r) = e.render_args() {
                self.world.add_resource(r);
                self.render_dispatcher.dispatch(&self.world.res);
                self.world.maintain();
            }

            // Wait 0.2s so we can see the changes on the visual debugger
            //let mut child = Command::new("sleep").arg("0.05").spawn().unwrap();
            //let _result = child.wait().unwrap();
        }
        println!("Showing results log...");
    }

    fn create_window() -> Window {
        WindowSettings::new("Simumo - Visual debugger", [1440, 2800])
            .opengl(Self::OPENGL_VERSION)
            .exit_on_esc(true)
            .build()
            .unwrap()
    }

    fn create_resources(world: &mut World, seed: String) {
        let graphics_handle = GlGraphics::new(Self::OPENGL_VERSION);
        let mut s: Uuid = Uuid::new_v4();

        world.add_resource(graphics_handle);

        world.add_resource(clock::Clock::new(0.25 * S));
        world.add_resource(generals::EndTime { val: 12.5 * S });

        if !seed.is_empty() {
            s = Uuid::parse_str(&seed)
                .unwrap_or_else(|_| panic!("Format of the seed isn't right."));
        }

        world.add_resource(s);
        world.add_resource(EventsManager::new());
        // For every entity, we define the entity it has to listen to, if any (this will be in a configuration file)
        {
            let mut events_manager = world.write_resource::<EventsManager>();
            // Here, for example, trafficlight2 observes trafficlight1
            events_manager.connect("trafficlight1".to_string(), "trafficlight2".to_string());
            // And here, trafficlight1 observes trafficlight2
            events_manager.connect("trafficlight2".to_string(), "trafficlight1".to_string());
        }
        world.add_resource(clock::Clock::new(0.25 * S));
        world.add_resource(generals::EndTime { val: 12.5 * S });

        //todo remove this and replace it by a config later one
        world.add_resource(LaneGraph::new(
            [
                (1, IntersectionData::new(10.0, 10.0)),
                (2, IntersectionData::new(10.0, 30.0)),
                (3, IntersectionData::new(20.0, 20.0)),
                (4, IntersectionData::new(30.0, 20.0)),
            ]
            .to_vec()
            .into_iter(),
            &[
                (1, 3, LaneData::new(None, None, None)),
                (2, 3, LaneData::new(None, None, None)),
                (3, 4, LaneData::new(None, None, None)),
            ],
        ));
    }
}

fn simulation_ended(ressources: &World) -> bool {
    // if keyboard end event  +

    let clock = ressources.read_resource::<clock::Clock>();
    let end_time = ressources.read_resource::<generals::EndTime>();
    clock.get_time() >= end_time.val
}
