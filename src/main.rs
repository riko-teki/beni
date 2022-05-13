use beni::ColorString;
use beni::colors::EightBitColors::*;
use beni::sys::csi;
fn main() {
    // if using on windows, necessary to execute following function. 
    // but, if using on linux, unnecessary.
    csi::enable_ansi_csi().expect("Failed to enable ansi csi.");

    let fg_g = "Hello".fg_color(Green);
    let bg_r = "Hello".bg_color(Red);
    let and = "Hello".bg_color(Red).fg_color(Green);
    let nand = "Hello".bg_color(Green).fg_color(Red).fg_color(Blue);
    println!("{}", fg_g);
    println!("{}", bg_r);
    println!("{}", and);
    println!("{}", nand);

}
