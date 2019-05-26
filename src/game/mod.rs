use tcod::input::{ Key };

use crate::util::Window;
use crate::npc::NPC;
use crate::character::Character;
use crate::render::TcodRender;
use crate::traits::{ Updates, RendererComponent };

pub struct Game {
    pub exit: bool,
    pub window: Window,
    pub render: TcodRender
}

impl Game{
    pub fn new() -> Game {
        let window = Window::new(80, 50);
        let render = TcodRender::new(&window);
        Game {
            exit: false,
            window,
            render 
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn render(&mut self, npcs: &Vec<NPC>, player: Character) {
        self.render.clear();
        for npc in npcs.iter() {
            npc.render(&mut self.render);
        }
        player.render(&mut self.render);
        self.render.flush();
    }

    pub fn update(&mut self, npcs: &mut Vec<NPC>, player: &mut Character, keypress: Key) {
        player.update(keypress, self);
        for npc in npcs.iter_mut() {
            npc.update(keypress, self);
        }
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