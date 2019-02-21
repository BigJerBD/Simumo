use specs::NullStorage;
use specs::WriteStorage;
use crate::graphics_00::Position;
use specs::{ ReadStorage, ReadExpect, Component, System };

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardMotionControl;

//Resource storing what buttons have been pressed
#[derive(Debug)]
pub struct ArrowKeysPressed {
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub down: bool
}

pub struct KeyboardSystem;
impl<'a> System<'a> for KeyboardSystem {
    type SystemData = (
        ReadExpect<'a, ArrowKeysPressed>,
        ReadStorage<'a, KeyboardMotionControl>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (buttons, _, mut pos_dat) = data;

        for pos in (&mut pos_dat).join() {
            if buttons.up {
                println!("FNEINVIRN");
                pos.y += 1.;
            }
            if buttons.left {
                pos.x -= 1.;
            }
            if buttons.right {
                pos.x += 1.;
            }
            if buttons.down {
                pos.y -= 1.;
            }
        }
    }
}