mod map;
mod player;

fn main() {
    let map = map::Map::parse(
        r#"
            ~~~~~~~~
            ~""""""~
            ~"$$$$"~
            ~"$nA$"~
            ~"$An$"~
            ~"$$$$"~
            ~""""""~
            ~~~~~~~~
        "#
    );

    cod::clear::all();
    map.print(0, 0);
    cod::flush();
}
