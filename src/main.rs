use beni::ColorString;

fn main() {
    //windows::enable_ansi_csi_on_windows().expect("Failed to enable ansi csi.");
    let b = "black".fg_black();
    let r = b.fg_red();
    let back = r.bg_blue();
    let j = back.fg_blue();
    println!("{}",j);
}
