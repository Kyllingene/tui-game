#![feature(try_trait_v2)]

mod entity;
mod input;
mod map;
mod player;
mod world;
mod quip;

use input::TurnResult;
use entity::EntityKind;
use map::{Tile, TileKind, Direction};

fn main() {
    let mut world = world::World::new(
        //            11111111112222
        //  012345678901234567890123
        r#"
            ~~~~~~~~~~~~~~~~~~~~~~~~
            ~~________~~~~~~~$nn$~~~
            ~____$$_$_____~~$$$nn$~~
            ~__$$$$$n$$$___~$$$An$~~
            ~~~~~$$$$$n$$_~~$$nn$_~~
            ~~~~~~~$$$$$$__~~nA$$_~~
            ~__~~~~A$$$$$__~~nn$$_~~
            ~_$_____~~~____~~$n$_~~~
            ~_$_____~~~~~~~~$nn$$_~~
            ~__nA$_________~$nA$$_~~
            ~____nnnnA_____~nn$$$$~~
            ~~____$$nn$__n__A$$$n$~~
            ~~~_____________$A$n$$~~
            ~~~___________~~~~~$$ ~~
            ~~~~~________~~~~~~~~~~~
            ~~~~~~~~~~~~~~~~~~~~~~~~
        "#,
        //            11111111112222
        //  012345678901234567890123
        12,
        14,
    );

    world.spawn(7, 7, EntityKind::Boss {
        health: 15,
        damage: 3,
        damage_gain: 2,
        id: 0,
        block: (Direction::Up, Tile { kind: TileKind::Grass }),
    });

    world.spawn(16, 12, EntityKind::Boss {
        health: 30,
        damage: 5,
        damage_gain: 2,
        id: 1,
        block: (Direction::Right, Tile { kind: TileKind::Forest }),
    });

    loop {
        cod::clear::all();
        world.draw(0, 0);

        cod::color::de();
        cod::goto::bot();
        cod::flush();

        let res = world.update();
        if res.bad() {
            world.draw_message(quip::random(res), 1);
            // world.draw_message(match res {
            //     TurnResult::Quit => "Goodbye, wimp",
            //     TurnResult::HungerDeath => "You died of hunger",
            //     TurnResult::ThirstDeath => "You died of thirst",
            //     TurnResult::ViolentDeath => "You died in glorious battle",
            //     _ => panic!("Unhandled bad outcome: {res:?}"),
            // }, 1);

            break;
        }
    }
}
