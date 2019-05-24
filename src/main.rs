use carakistan::util::{Point, Bound};
use carakistan::traits::Updates;
use carakistan::game::Game;
use carakistan::character::Character;
use carakistan::npc::NPC;
use tcod::console::*;
use tcod::input::{ Key, KeyCode };
use tcod::{ RootConsole };

fn main() {
    let mut game = Game {
        exit: false,
        window_bounds: Bound {
            min: Point { x: 0, y: 0 },
            max: Point { x: 79, y: 49 } 
        }
    };
    
    let mut con: RootConsole = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(game.window_bounds.max.x + 1, game.window_bounds.max.y + 1)
        .title("Rust/libtcod tutorial")
        .init();

    let mut player = Character::new(40, 25, '@');
    let dog = Box::new(NPC::new(10, 10, 'd')) as Box<Updates>;
    let cat = Box::new(NPC::new(40, 25, 'c')) as Box<Updates>;

    let mut npcs: Vec<Box<Updates>> = vec![cat, dog];

    render(&mut con, &mut npcs, player);
    while !con.window_closed() && !&game.exit {
        let keypress: Key = con.wait_for_keypress(true);

        match keypress.code { 
            KeyCode::Escape => game.exit = true,
            _ => ()
        }

        update(&mut npcs, &mut player, keypress, &game);
        render(&mut con, &mut npcs, player);
    }
}

fn render(con: &mut RootConsole, objs: &mut Vec<Box<Updates>>, player: Character) {
    con.clear();
    for obj in objs.iter() {
        obj.render(con);
    }
    player.render(con);
    con.flush();
}

fn update(objs: &mut Vec<Box<Updates>>, player: &mut Character, keypress: Key, game: &Game) {
    player.update(keypress, game);
    for obj in objs.iter_mut() {
        obj.update(keypress, game);
    }
}