use tcod::console::{Root};
use tcod::input::{Key};
use crate::game::Game;

pub trait Updates{
    fn update(&mut self, keypress: Key, game: &Game);
    fn render(&self, console: &mut Root);
}