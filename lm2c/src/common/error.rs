use crate::common::{Color, Loc};

#[allow(dead_code)]
pub fn assembler_error(msg: &str) {
    println!("{bold}{red}error{reset}: {msg}", 
        // underline=Color::UNDERLINE
        bold=Color::BOLD,
        red=Color::RED,
        reset=Color::RESET
    )
}

#[allow(dead_code)]
pub fn assembler_loc_error(loc: &Loc, msg: &str) {
    println!("{underline}{loc}:{reset}{bold}{red}error{reset}: {msg}", 
        underline=Color::UNDERLINE,
        loc=loc.to_human(),
        bold=Color::BOLD,
        red=Color::RED,
        reset=Color::RESET
    )
}

#[allow(dead_code)]
pub fn assembler_warn(msg: &str) {
    println!("{bold}{yellow}warning{reset}: {msg}", 
        // underline=Color::UNDERLINE
        bold=Color::BOLD,
        yellow=Color::YELLOW,
        reset=Color::RESET
    )
}

#[allow(dead_code)]
pub fn assembler_loc_warn(loc: &Loc, msg: &str) {
    println!("{underline}{loc}:{reset}{bold}{yellow}warning{reset}: {msg}", 
        underline=Color::UNDERLINE,
        loc=loc.to_human(),
        bold=Color::BOLD,
        yellow=Color::YELLOW,
        reset=Color::RESET
    )
}