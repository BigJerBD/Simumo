use std::collections::HashMap;
use std::path::Path;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::GlGraphics;
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use piston_window::OpenGL;
use piston_window::RenderEvent;
use specs::prelude::{DispatcherBuilder, World};
use specs::Dispatcher;
use uuid::Uuid;

use crate::configurations::generals::EndTime;
use crate::configurations::generals::VisualDebugger;
use crate::configurations::Configuration;
use crate::entities::entity_type::Instantiable;
use crate::ressources::clock;
use crate::ressources::eventsmanagement::EventsManager;
use crate::ressources::random::Random;
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
        let width: f64 = config.generals.debugger.width;
        let height: f64 = config.generals.debugger.height;

        let window = Self::create_window(width, height);
        //let img = Image::new().rect(square(0.0, 0.0, 200.0));
        //img.save("build/testimage.png").unwrap();
        //image::save_buffer(&Path::new("build/testimage.png"), img, 800, 600, image::RGBA(8));
        create_background_image(width, height);

        let mut base_dispatcher_builder = DispatcherBuilder::new();
        let mut world = World::new();
        let mut system_mapping = HashMap::<String, Vec<String>>::new();

        Self::create_ressources(&mut world, &config);

        config.systems.declare_systems(&mut system_mapping);
        add_starting_systems(&mut base_dispatcher_builder);
        config
            .systems
            .setup_systems(&mut base_dispatcher_builder, &system_mapping);
        add_ending_systems(&mut base_dispatcher_builder);

        let mut base_dispatcher = base_dispatcher_builder.build();
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

    fn create_window(width: f64, height: f64) -> Window {
        WindowSettings::new("Simumo - Visual debugger", [width, height])
            .opengl(Self::OPENGL_VERSION)
            .exit_on_esc(true)
            .build()
            .unwrap()
    }
    //
    ///Create default world's ressources and config's ressources
    fn create_ressources(world: &mut World, config: &Configuration) {
        let graphics_handle = GlGraphics::new(Self::OPENGL_VERSION);
        let end_time = config.generals.end_time.clone();
        let debugger = config.generals.debugger.clone();
        let seed = if !config.generals.seed.is_empty() {
            Uuid::parse_str(&config.generals.seed).unwrap_or_else(|_| panic!("invalid seed format"))
        } else {
            Uuid::new_v4()
        };
        let random = Random::from_uuid(&seed);

        config.map.forward_ressources(world);
        world.add_resource(end_time);
        world.add_resource(graphics_handle);
        world.add_resource(clock::Clock::new(config.generals.clock_dt));
        world.add_resource(EventsManager::new());
        world.add_resource(debugger);
        world.add_resource(random);
    }
}

fn simulation_ended(ressources: &World) -> bool {
    // if keyboard end event  +
    let clock = ressources.read_resource::<clock::Clock>();
    let end_time = ressources.read_resource::<EndTime>();
    clock.get_time() >= end_time.val
}

fn create_background_image(width: f64, height: f64) {
    let width: u32 = width as u32;
    let height: u32 = height as u32;
    let path = Path::new("build/background.png");
    /*
    extern crate image;
    extern crate imageproc;
    use image::{Rgba, RgbaImage};
    use imageproc::drawing::draw_filled_rect_mut;
    let color_bg = Rgba([51u8, 135u8, 31u8, 255u8]);
    let color_street = Rgba([255u8, 0u8, 0u8, 127u8]);
    let mut img = RgbaImage::new(width, height);
    // Draw background color
    draw_filled_rect_mut(
        &mut img,
        imageproc::rect::Rect::at(0, 0).of_size(width, height),
        color_bg
    );
    // Draw streets
    draw_filled_rect_mut(
        &mut img,
        imageproc::rect::Rect::at(50, 50).of_size(50, 50),
        color_street
    );
    img.save(path).unwrap();*/
    use graphics::*;
    use graphics_buffer::*;
    let color_bg = [0.2, 0.53, 0.12, 1.0];
    let street_bg = [0.75, 0.75, 0.75, 1.0];
    let mut buffer = RenderBuffer::new(width, height);
    buffer.clear(color_bg);

    let p1: (f64, f64) = (3.0, 20.0);
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
    );

    // Big red circle
    /*ellipse(
        [1.0, 0.0, 0.0, 0.7],
        [0.0, 0.0, 100.0, 100.0],
        IDENTITY,
        &mut buffer,
    );
    // Small blue circle
    ellipse(
        [0.0, 0.0, 1.0, 0.7],
        [0.0, 0.0, 50.0, 50.0],
        IDENTITY,
        &mut buffer,
    );
    // Small green circle
    ellipse(
        [0.0, 1.0, 0.0, 0.7],
        [50.0, 50.0, 50.0, 50.0],
        IDENTITY,
        &mut buffer,
    );*/

    // Save the buffer
    buffer.save("build/circles.png").unwrap();
}
