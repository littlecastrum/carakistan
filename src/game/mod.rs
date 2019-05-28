use tcod::input::{ Key };

use crate::util::{ Point, Window};
use crate::render::{ RendererComponent, TcodRender };
use crate::character::Character;

static mut LAST_KEYPRESS : Option<Key> = None;
static mut HERO_LOCATION : Point = Point { x: 40, y: 25 };

pub struct Game {
    pub exit: bool,
    pub window: Window,
    pub render: Box<RendererComponent>
}

impl Game {
    pub fn new() -> Game {
        let window = Window::new(80, 50);
        let render = Box::new(TcodRender::new(&window));
        Game {
            exit: false,
            window,
            render 
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn render(&mut self, npcs: &Vec<Character>, player: &Character) {
        self.render.clear();
        for npc in npcs.iter() {
            npc.render(&mut self.render);
        }
        player.render(&mut self.render);
        self.render.flush();
    }

    pub fn update(&mut self, npcs: &mut Vec<Character>, player: &mut Character) {
        player.update();
        Game::set_hero_point(player.position);
        for npc in npcs.iter_mut() {
            npc.update();
        }
    }

    pub fn get_last_keypress() -> Option<Key> {
        unsafe { LAST_KEYPRESS }
    }

    pub fn set_last_keypress(keypress: Key) {
        unsafe { LAST_KEYPRESS = Some(keypress); }
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        let keypress = self.render.wait_for_keypress();
        Game::set_last_keypress(keypress);
        return keypress;
    }

    pub fn get_hero_point() -> Point {
        unsafe { HERO_LOCATION }
    }

    pub fn set_hero_point(point: Point) {
        unsafe { HERO_LOCATION = point; }
    }

    pub fn window_closed(&mut self) -> bool {
        self.render.window_closed()
    }
}

#[cfg(test)]
mod tests {
    use super::Window;
    use super::Game;

    #[test]
    fn new_game() {
        let game = Game::new();
        assert_eq!(game.exit, false);
        assert_eq!(game.window, Window { width: 80, height: 50 });
    }

    #[test]
    fn exit_game() {
        let mut game = Game::new();
        assert_eq!(game.exit, false);
        game.exit();
        assert_eq!(game.exit, true);
    }
}