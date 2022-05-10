// Reference https://docs.microsoft.com/en-us/windows/console/getconsolemode
const ENABLE_VIRTUAL_TERMINAL_PROCESING: u32 = 0x0004;

#[cfg(target_os = "windows")]
pub fn enable_ansi_csi_on_windows() -> Result<(), std::io::Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::fileapi::{CreateFileW, OPEN_EXISTING};
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::winnt::{FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE};

    unsafe {
        let console_out_name: Vec<u16> =
            OsStr::new("CONOUT$").encode_wide().chain(once(0)).collect();
        let console_handle = CreateFileW(
            console_out_name.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_WRITE,
            null_mut(),
            OPEN_EXISTING,
            0,
            null_mut(),
        );
        if console_handle == INVALID_HANDLE_VALUE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid handle value",
            ));
        }

        let mut console_mode: u32 = 0;

        if 0 == GetConsoleMode(console_handle, &mut console_mode) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("GetConsoleMode failed! error code: {}", GetLastError()),
            ));
        }

        if console_mode & ENABLE_VIRTUAL_TERMINAL_PROCESING == 0 {
            if 0 == SetConsoleMode(
                console_handle,
                console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESING,
            ) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("SetConsoleMode failed! error code: {}", GetLastError()),
                ));
            }
        }
        Ok(())
    }
}
