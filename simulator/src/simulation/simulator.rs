use dim::si::M;
use dim::si::S;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::GlGraphics;
use piston::event_loop::{Events, EventSettings};
use piston::window::WindowSettings;
use piston_window::OpenGL;
use piston_window::RenderEvent;
use specs::Dispatcher;
use specs::prelude::*;
use uuid::Uuid;

use crate::configurations::Configuration;
use crate::configurations::map;
use crate::entities::entity_type::Instantiable;
use crate::ressources::clock;
use crate::ressources::eventsmanagement::EventsManager;
use crate::ressources::generals;
use crate::ressources::lane_graph::IntersectionData;
use crate::ressources::lane_graph::LaneData;
use crate::ressources::lane_graph::LaneGraph;
use crate::simulation::dispatchers::make_base_dispatcher;
use crate::simulation::dispatchers::make_render_dispatcher;
//use std::process::Command;

pub struct Simulation<'a, 'b> {
    world: World,
    window: Window,
    base_dispatcher: Dispatcher<'a, 'b>,
    render_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Simulation<'a, 'b> {
    const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

    pub fn from_config(config: Configuration) -> Self {
        //Todo: Systems

        let mut world = World::new();
        let window = Self::create_window();

        //ressources
        Self::create_config_ressource(&mut world, &config);
        Self::create_ressources(&mut world);

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

    fn create_config_ressource(world: &mut World, config: &Configuration) {
        let seed = if !config.generals.seed.is_empty() {
            Uuid::parse_str(&config.generals.seed)
                .unwrap_or_else(|_| panic!("invalid seed format"))
        } else {
            Uuid::new_v4()
        };

        world.add_resource(seed);

        // TODO: J'ai commenté ici vu que je suis incapable de tester autrement (problèmes avec Python)
        // À décommenter éventuellement
        /*match &config.map {
            map::Map::OsmGraph(val) => world.add_resource(LaneGraph::from_pyosmgraph(
                val.longitude,
                val.latitude,
                val.zoom,
            )),
        };*/
    }

    fn create_ressources(world: &mut World) {
        let graphics_handle = GlGraphics::new(Self::OPENGL_VERSION);
        world.add_resource(graphics_handle);
        world.add_resource(clock::Clock::new(0.25 * S));
        world.add_resource(generals::EndTime { val: 12.5 * S });
        world.add_resource(EventsManager::new());
        // For every entity, we define the entity it has to listen to, if any (this will be in a configuration file)
        //todo make this properly configurable
        {
            let mut events_manager = world.write_resource::<EventsManager>();
            // Here, for example, trafficlight2 observes trafficlight1
            events_manager.connect("trafficlight01".to_string(), "trafficlight02".to_string());
            // And here, trafficlight1 observes trafficlight2
            events_manager.connect("trafficlight02".to_string(), "trafficlight01".to_string());
        }
        world.add_resource(clock::Clock::new(0.25 * S));
        world.add_resource(generals::EndTime { val: 12.5 * S });

        //todo remove this and replace it by a config later one
        // TODO: J'ai décommenté ici pour avoir un LaneGraph simple de test
        world.add_resource(LaneGraph::new(
            [
                (1, IntersectionData::new(10.0, 10.0)),
                (2, IntersectionData::new(50.0, 240.0)),
                (3, IntersectionData::new(150.0, 100.0)),
                (4, IntersectionData::new(400.0, 100.0)),
            ]
            .to_vec()
            .into_iter(),
            &[
                (1, 3, LaneData::new(Some(3.0 * M), None, None)),
                (2, 3, LaneData::new(Some(2.5 * M), None, None)),
                (3, 4, LaneData::new(Some(3.5 * M), None, None)),
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
