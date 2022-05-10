use beni::sys::windows;
mod csi;

fn main() {
    windows::enable_ansi_csi_on_windows();
    println!("\x1b[48;5;100mHello World\x1b[m");
}
