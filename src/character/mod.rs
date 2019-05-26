use tcod::input::{ Key, KeyCode };

use crate::traits::{ Updates, RendererComponent };
use crate::util::{ Point, Contains::{ DoesContain, DoesNotContain } };
use crate::game::Game;
use crate::render::TcodRender;

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

        match game.window.contains(self.position.offset(offset)) {
            DoesContain => self.position = self.position.offset(offset),
            DoesNotContain => ()
        }
    }

    fn render(&self, console: &mut TcodRender) {
        console.add_object(self.position, self.display_char);
    }
}


#[cfg(test)]
mod tests {
    use super::Character;
    use crate::util::{ Point };

    #[test]
    fn new_character() {
        let player = Character::new(40, 25, '@');
        assert_eq!(player.position, Point { x: 40, y: 25 });
        assert_eq!(player.display_char, '@');
    }
}