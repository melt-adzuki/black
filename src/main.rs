mod msg;

use msg::Msg;
use std::{thread, time::Duration};
use windows::{
    core::*, Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        let msg = Msg {
            hwnd: GetForegroundWindow(),
            msg: WM_SYSCOMMAND,
            wparam: WPARAM(SC_MONITORPOWER as usize),
            lparam: LPARAM(2),
        };

        thread::sleep(Duration::from_secs(1));
        msg.post_message_w();
    }

    Ok(())
}
