use tcod::input::{ KeyCode };

use carakistan::game::Game;
use carakistan::character::Character;

fn main() {
    let mut game = Game::new();
    let mut player = Character::hero(game.window);
    let mut npcs: Vec<Character> = vec![
        Character::dog(10, 10, game.window),
        Character::cat(40, 25, game.window),
        Character::kobold(40, 25, game.window)
    ];

    game.render(&mut npcs, &player);
    while !(game.window_closed() || game.exit) {
        let keypress = game.wait_for_keypress();

        match keypress.code { 
            KeyCode::Escape => game.exit(),
            _ => ()
        }
        
        game.update(&mut npcs, &mut player);
        game.render(&mut npcs, &player);
    }
}