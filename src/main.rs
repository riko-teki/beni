use beni::sys::windows;
use beni::colors;

fn main() {
    windows::enable_ansi_csi_on_windows().expect("Failed to enable ansi csi.");
    println!("\x1b[48;5;100mHello World\x1b[m");
}
