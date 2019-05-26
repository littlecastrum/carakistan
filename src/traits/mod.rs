use tcod::input::{ Key };
use crate::game::Game;
use crate::render::TcodRender;
use crate::util::{ Point, Window };

pub trait Updates{
    fn update(&mut self, keypress: Key, game: &Game);
    fn render(&self, console: &mut TcodRender);
}

pub trait RendererComponent {
    fn new(console: &Window) -> Self;
    fn clear(&mut self);
    fn add_object(&mut self, position: Point, symbol: char);
    fn flush(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn window_closed(&mut self) -> bool;
}