use crate::render::RendererComponent;
use crate::movement::{ MovementComponent, RandomMovementComponent, PlayerMovementComponent };
use crate::util::{ Point, Window };
use crate::game::Game;

pub struct Character {
    pub position: Point,
    pub display_char: char,
    pub movement_component: Box<MovementComponent>
}

impl Character {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent>) -> Character {
        Character {
            position: Point { x, y },
            display_char: dc,
            movement_component: mc
        }
    }
    
    pub fn update(&mut self) {
        self.position = self.movement_component.update(self.position);
    }

    pub fn render(&self, console: &mut Box<RendererComponent>) {
        console.add_object(self.position, self.display_char);
    }

    pub fn dog(x: i32, y: i32, window: Window) -> Character {
        let mc : Box<MovementComponent> = Box::new(RandomMovementComponent::new(window));
        Character::new(x, y, 'd', mc)
    }

    pub fn cat(x: i32, y: i32, window: Window) -> Character {
        let mc : Box<MovementComponent> = Box::new(RandomMovementComponent::new(window));
        Character::new(x, y, 'c', mc)
    }

    pub fn kobold(x: i32, y: i32, window: Window) -> Character {
        let mc : Box<MovementComponent> = Box::new(RandomMovementComponent::new(window));
        Character::new(x, y, 'k', mc)
    }

    pub fn hero(window: Window) -> Character {
        let point = Game::get_hero_point();
        let mc : Box<MovementComponent> = Box::new(PlayerMovementComponent::new(window));
        Character::new(point.x, point.y, '@', mc)
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