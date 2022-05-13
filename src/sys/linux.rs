#[cfg(target_os = "linux")]
pub fn enable_ansi_csi() -> Result<(), std::io::Error> {
    // NOP
    Ok(())
}
