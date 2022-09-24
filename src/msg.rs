use windows::Win32::{Foundation::*, UI::WindowsAndMessaging::*};

pub struct Msg {
    pub hwnd: HWND,
    pub msg: u32,
    pub wparam: WPARAM,
    pub lparam: LPARAM,
}

impl Msg {
    pub unsafe fn post_message_w(&self) {
        PostMessageW(self.hwnd, self.msg, self.wparam, self.lparam);
    }
}
