use tcod::input::{ KeyCode };

use carakistan::game::Game;
use carakistan::character::Character;
use carakistan::npc::NPC;
use carakistan::traits::{ RendererComponent };

fn main() {
    let mut game = Game::new();
    let mut player = Character::new(40, 25, '@');
    let mut npcs: Vec<NPC> = vec![
        NPC::new(10, 10, 'd'),
        NPC::new(40, 25, 'c')
    ];

    game.render(&mut npcs, player);
    while !(game.window_closed() || game.exit) {
        let keypress = game.render.wait_for_keypress();

        match keypress.code { 
            KeyCode::Escape => game.exit(),
            _ => ()
        }
        
        game.update(&mut npcs, &mut player, keypress);
        game.render(&mut npcs, player);
    }
}