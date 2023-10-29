mod map;
mod player;

fn main() {
    let map = map::Map::parse(
        r#"
            ~~~~~~~~
            ~""""""~
            ~"!!!!"~
            ~"!nA!"~
            ~"!An!"~
            ~"!!!!"~
            ~""""""~
            ~~~~~~~~
        "#
    );

    // dbg!(map);
    map.print();
}
