extern crate libc;
extern crate widestring;

use std::ptr::null_mut;

mod win32;

pub unsafe extern fn window_proc(
  hwnd   : win32::HWND,
  uMsg   : win32::UINT,
  wParam : win32::WPARAM,
  lParam : win32::LPARAM) -> win32::LRESULT
{
    return 0;
}


fn main()
{
    unsafe {
        let class_name = win32::wide_string("red");

        let wc = win32::WNDCLASS {
            lpfnWndProc   : Some(window_proc),
            lpszClassName : class_name,
            ..Default::default()
        };

        win32::RegisterClassW(&wc);

        let hwnd = win32::CreateWindowExW(
            0,
            class_name,
            class_name,
            win32::WS_TILED | win32::WS_VISIBLE,
            0, 0,
            1280, 720,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut());
    }

    loop {
    }

    println!("hello world!");
}
