const TITLE: &str = include_str!("../title/title.txt");
const LOGO: &str = include_str!("../title/logo.txt");
const LOGO_SHADING: &str = include_str!("../title/logo_shading.txt");

pub fn draw(x: u32, y: u32) {
    cod::clear::all();
    title(x, y);
    logo(x + 24, y + 2);

    cod::color::de();
    cod::goto::pos(x + 2, y + 6);
    println!("=\\ Press any    |");
    cod::goto::right(x + 2);
    println!(" | key to begin |");
}

fn title(x: u32, y: u32) {
    let title = TITLE.replace('#', "\t");

    cod::color::fg(238);
    cod::blit_transparent(&title, x + 2, y + 1);
    cod::color::de_fg();
    cod::blit_transparent(&title, x, y);
}

fn logo(x: u32, y: u32) {
    cod::color::fg(28);
    cod::blit(LOGO, x, y);
    cod::color::fg(22);
    cod::blit_transparent(LOGO_SHADING, x, y);
}
