#![feature(try_trait_v2)]

mod entity;
mod input;
mod map;
mod player;
mod quip;
mod sector;
mod title;
mod world;
mod world_map;

fn main() {
    title::draw(2, 2);
    cod::read::key();

    let mut world = world::World::new(
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
            world.draw(0, 0);
            world.draw_message(quip::random(res), 1);
            break;
        }
    }

    cod::goto::bot();
    cod::goto::up(1);
    println!("   Press any key to exit   ");
    cod::read::key();
}
