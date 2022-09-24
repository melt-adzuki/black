use windows::{
    core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        let hwnd = GetForegroundWindow();
        let msg = WM_SYSCOMMAND;
        let wparam = WPARAM(SC_MONITORPOWER as usize);
        let lparam = LPARAM(2);

        PostMessageW(hwnd, msg, wparam, lparam);
    }

    Ok(())
}
