use beni::ColorString;
use beni::colors::EightBitColors::*;

fn main() {
    //windows::enable_ansi_csi_on_windows().expect("Failed to enable ansi csi.");
    let fg_g = "Hello".fg_color(Green);
    let bg_r = "Hello".bg_color(Red);
    let and = "Hello".bg_color(Red).fg_color(Green);
    let nand = "Hello".bg_color(Green).fg_color(Red).fg_color(Blue);
    println!("{}", fg_g);
    println!("{}", bg_r);
    println!("{}", and);
    println!("{}", nand);
    //println!("{}", s.fg_color(s, Red));
}
