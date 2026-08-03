/*
pub const : &str = "\x1b[";
*/
pub const SMCUP: &str = "\x1b[?1049h";
pub const RMCUP: &str = "\x1b[?1049l";
pub const CIVIS: &str = "\x1b[?25l";
pub const CNORM: &str = "\x1b[?25h";
pub const RSTCLR: &str = "\x1b[0m";
pub const SAVCUR: &str = "\x1b7";
pub const RESTORCUR: &str = "\x1b8";
pub const GETCURPOS: &str = "\x1b[6n";
pub const CURHOME: &str = "\x1b[H";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const STRIKETHROUGH: &str = "\x1b[9m";
pub const CLEAR: &str = "\x1b[2J";
pub const CURUP: &str = "\x1b[1A";
pub const CURDOWN: &str = "\x1b[1B";
pub const CURRIGHT: &str = "\x1b[1C";
pub const CURLEFT: &str = "\x1b[1D";

pub fn rgb_foreground(r: u8, g: u8, b: u8) -> String
{
    return format!("\x1b[38;2;{};{};{}m", r, g, b);
}

pub fn rgb_background(r: u8, g: u8, b: u8) -> String
{
    return format!("\x1b[48;2;{};{};{}m", r, g, b);
}

pub fn cursor_pos(x: u16, y: u16) -> String
{
    if(x == 0 || y == 0)
    {
        panic!("Position cannot be less than (1, 1)");
    }

    return format!("\x1b[{};{}H", y, x);
}