use std::cmp;
use std::collections::HashMap;
use std::f64::INFINITY;
use std::f64::NEG_INFINITY;

use dim::si::{M, S};
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
use crate::ressources::clock;
use crate::ressources::eventsmanagement::EventsManager;
use crate::ressources::generals;
use crate::ressources::generals::MapBbox;
use crate::ressources::lane_graph::IntersectionData;
use crate::ressources::lane_graph::LaneData;
use crate::ressources::lane_graph::LaneGraph;
use crate::simulation::dispatchers::add_ending_systems;
use crate::simulation::dispatchers::add_starting_systems;
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
        let mut world = World::new();
        let window = Self::create_window();

        //ressources
        Self::create_config_ressource(&mut world, &config);
        Self::create_ressources(&mut world);

        let mut system_mapping = HashMap::<String, Vec<String>>::new();
        let mut base_dispatcher_builder = DispatcherBuilder::new();

        config.systems.declare_systems(&mut system_mapping);
        add_starting_systems(&mut base_dispatcher_builder);
        config
            .systems
            .setup_systems(&mut base_dispatcher_builder, &system_mapping);
        add_ending_systems(&mut base_dispatcher_builder);

        let mut render_dispatcher = make_render_dispatcher();
        let mut base_dispatcher = base_dispatcher_builder.build();

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
        WindowSettings::new("Simumo - Visual debugger", [1440, 1440])
            .opengl(Self::OPENGL_VERSION)
            .exit_on_esc(true)
            .build()
            .unwrap()
    }

    fn create_config_ressource(world: &mut World, config: &Configuration) {
        let seed = if !config.generals.seed.is_empty() {
            Uuid::parse_str(&config.generals.seed).unwrap_or_else(|_| panic!("invalid seed format"))
        } else {
            Uuid::new_v4()
        };

        world.add_resource(seed);

        // todo move this crate::configurations::map
        //match &config.map {
        //    map::Map::OsmGraph(val) => add_lane_graph(
        //        LaneGraph::from_pyosmgraph(val.longitude,val.latitude,val.zoom),
        //        world
        //    ),
        //};
    }

    fn create_ressources(world: &mut World) {
        let graphics_handle = GlGraphics::new(Self::OPENGL_VERSION);
        world.add_resource(graphics_handle);
        world.add_resource(clock::Clock::new(0.25 * S));
        world.add_resource(generals::EndTime { val: 12.5 * S });
        world.add_resource(EventsManager::new());
        // TODO remove when not needed anymore
        add_lane_graph(
            LaneGraph::new(
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
            ),
            world,
        );
    }
}

// todo move this function in crate::configurations::map
fn add_lane_graph(lanegraph: LaneGraph, world: &mut World) {
    let positions: Vec<(f64, f64)> = lanegraph
        .intersections
        .values()
        .map(|v| v.position())
        .collect();

    //this is quite long we can find interesting way to simplify this
    let padding = 10.0;
    world.add_resource(MapBbox {
        x1: positions
            .iter()
            .map(|v| v.0.clone())
            .fold(0. / 0., f64::min)
            + padding,
        x2: positions
            .iter()
            .map(|v| v.0.clone())
            .fold(0. / 0., f64::max)
            - padding,
        y1: positions
            .iter()
            .map(|v| v.1.clone())
            .fold(0. / 0., f64::min)
            + padding,
        y2: positions
            .iter()
            .map(|v| v.1.clone())
            .fold(0. / 0., f64::max)
            - padding,
    });
    world.add_resource(lanegraph);
}

fn simulation_ended(ressources: &World) -> bool {
    // if keyboard end event  +
    let clock = ressources.read_resource::<clock::Clock>();
    let end_time = ressources.read_resource::<generals::EndTime>();
    clock.get_time() >= end_time.val
}
