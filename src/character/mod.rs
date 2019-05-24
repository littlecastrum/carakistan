use tcod::console::{ Root, Console,  BackgroundFlag };
use tcod::input::{ Key, KeyCode };

use crate::traits::Updates;
use crate::util::{Point, Contains::{DoesContain, DoesNotContain}};
use crate::game::Game;

#[derive(Debug, Copy, Clone)]
pub struct Character {
    pub position: Point,
    pub display_char: char
}

impl Character {
    pub fn new(x: i32, y: i32, dc: char) -> Character {
        Character {
            position: Point { x, y },
            display_char: dc
        }
    }
}

impl Updates for Character {
    fn update(&mut self, keypress: Key, game: &Game) {
        let mut offset = Point { x: 0, y: 0 };

        match keypress.code {
            KeyCode::Up => offset.y = -1,
            KeyCode::Down => offset.y = 1,
            KeyCode::Left => offset.x = -1,
            KeyCode::Right => offset.x = 1,
            _ => ()
        }

        match game.window_bounds.contains(self.position.offset(offset)) {
            DoesContain => self.position = self.position.offset(offset),
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