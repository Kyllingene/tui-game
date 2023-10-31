#![feature(try_trait_v2)]

mod entity;
mod input;
mod map;
mod player;
mod world;

use input::TurnResult;
use map::{WIDTH, HEIGHT};

fn main() {
    let mut world = world::World::new(
        r#"
            ~~~~~~~~~~~~~~~~~~~~~~~~
            ~~""""""""~~~~~~~$nn$~~~
            ~""""$$"$"""""~~$$$nn$~~
            ~""$$$$$n$$$"""~$$$An$~~
            ~~~~~$$$$$n$$"~~$$nn$"~~
            ~~~~~~~$$$$$$""~~nA$$"~~
            ~""~~~~"$$$$$""~~nn$$"~~
            ~"$"""""~~~""""~~$n$"~~~
            ~"$"""""~~~~~~~~$nn$$"~~
            ~""nA$"""""""""~$nA$$"~~
            ~""""nnnnA"""""~nn$$$$~~
            ~~""""$$nn$""n""n$$$n$~~
            ~~~"""""""""""""$$$n$$~~
            ~~~""""""""""~~~~~~$$ ~~
            ~~~~~""""""""~~~~~~~~~~~
            ~~~~~~~~~~~~~~~~~~~~~~~~
        "#,
        12,
        14,
    );

    loop {
        cod::clear::all();
        world.draw(0, 0);

        cod::color::de();
        cod::goto::bot();
        cod::flush();

        let res = world.update();
        if res.bad() {
            let msg = match res {
                TurnResult::Quit => "Goodbye, wimp",
                TurnResult::HungerDeath => "You died by hunger",
                TurnResult::ThirstDeath => "You died by thirst",
                TurnResult::ViolentDeath => "You died in battle",
                _ => panic!("Unhandled bad outcome: {res:?}"),
            };

            let x = (WIDTH / 2).saturating_sub(msg.len() / 2);
            let y = HEIGHT / 2 - 1;

            cod::color::de();
            cod::color::fg(1);
            cod::goto::pos(x as u32, y as u32);
            cod::clear::line();
            print!("{msg}");
            cod::goto::bot();
            cod::flush();

            break;
        }
    }
}
