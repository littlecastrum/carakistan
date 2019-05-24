use tcod::console::{ Root, Console, BackgroundFlag };
use tcod::input::{ Key };
use rand::Rng;

use crate::traits::Updates;
use crate::util::{Point, Contains::{DoesContain, DoesNotContain}};
use crate::game::Game;

pub struct NPC {
    pub position:     Point,
    pub display_char: char
}

impl NPC {
    pub fn new(x: i32, y: i32, dc: char) -> NPC {
        NPC {
            position: Point { x, y },
            display_char: dc
        }
    }
}

impl Updates for NPC {
    fn update(&mut self, _keypress: Key, game: &Game) {
        let offset_x = rand::thread_rng().gen_range(0, 3) - 1;
        match game.window_bounds.contains(self.position.offset_x(offset_x)) {
            DoesContain => self.position = self.position.offset_x(offset_x),
            DoesNotContain => ()
        }

        let offset_y = rand::thread_rng().gen_range(0, 3) - 1;
        match game.window_bounds.contains(self.position.offset_y(offset_y)) {
            DoesContain => self.position = self.position.offset_y(offset_y),
            DoesNotContain => ()
        }
    }

    fn render(&self, console: &mut Root) {
        console.put_char(
            self.position.x,
            self.position.y,
            self.display_char,
            BackgroundFlag::Set
        );
    }
}
