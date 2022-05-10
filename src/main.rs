use beni::colors::{self, EightBitColors};
use beni::sys::windows;
use beni::Color;

fn main() {
    //windows::enable_ansi_csi_on_windows().expect("Failed to enable ansi csi.");
    println!("{}", "black".black());
    println!("{}", "red".red());
    println!("{}", "bg_red".bg_color(EightBitColors::Red));
    println!("\x1b[48;5;76m\x1b[48;5;150mHello World\x1b[m");
}
