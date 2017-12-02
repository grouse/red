extern crate libc;
extern crate widestring;

use std::ptr::null_mut;

mod win32 {
    use std;
    use libc;
    use widestring::WideCString;

    pub const WS_TILED   : u32 = 0x00000000;
    pub const WS_VISIBLE : u32 = 0x10000000;

    pub type LPVOID    = *mut std::os::raw::c_void;
    pub type PVOID     = LPVOID;
    pub type HANDLE    = PVOID;
    pub type HWND      = HANDLE;
    pub type HMENU     = HANDLE;
    pub type HINSTANCE = HANDLE;
    pub type HICON     = HANDLE;
    pub type HCURSOR   = HANDLE;
    pub type HBRUSH    = HANDLE;

    pub type WORD  = u16;
    pub type DWORD = u32;

    pub type BOOL  = i32;
    pub type UINT  = u32;
    pub type INT   = u32;

    pub type ATOM = WORD;

    pub type LONG_PTR  = libc::intptr_t;
    pub type ULONG_PTR = libc::uintptr_t;

    pub type WPARAM = libc::uintptr_t;
    pub type LPARAM = LONG_PTR;

    pub type LRESULT = LONG_PTR;

    pub type LPCWSTR   = *const u16;
    pub type LPWSTR   =  *mut   u16;

    pub fn wide_string(s : &str) -> LPWSTR
    {
        return WideCString::from_str(s).unwrap().into_raw();
    }


    type WNDPROC = unsafe extern fn (
        hwnd   : HWND,
        uMsg   : UINT,
        wParam : WPARAM,
        lParam : LPARAM) -> LRESULT;

    #[repr(C)]
    pub struct WNDCLASS {
        pub style         : UINT,
        pub lpfnWndProc   : Option<WNDPROC>,
        pub cbClsExtra    : INT,
        pub cbWndExtra    : INT,
        pub hInstance     : HINSTANCE,
        pub hIcon         : HICON,
        pub hCursor       : HCURSOR,
        pub hbrBackground : HBRUSH,
        pub lpszMenuName  : LPCWSTR,
        pub lpszClassName : LPCWSTR
    }

    impl Default for WNDCLASS {
        fn default() -> WNDCLASS {
            WNDCLASS {
                style         : 0 as UINT,
                lpfnWndProc   : None,
                cbClsExtra    : 0 as INT,
                cbWndExtra    : 0 as INT,
                hInstance     : 0 as HINSTANCE,
                hIcon         : 0 as HICON,
                hCursor       : 0 as HCURSOR,
                hbrBackground : 0 as HBRUSH,
                lpszMenuName  : 0 as LPCWSTR,
                lpszClassName : 0 as LPCWSTR,
            }
        }
    }

    #[link(name = "user32")]
    extern  {
        pub fn CreateWindowExW(
            dwExStyle    : DWORD,
            lpClassName  : LPCWSTR,
            lpWindowName : LPCWSTR,
            dwStyle      : DWORD,
            x            : INT,
            y            : INT,
            nWidth       : INT,
            nHeight      : INT,
            hWndParent   : HWND,
            hMenu        : HMENU,
            hInstance    : HINSTANCE,
            lpParam      : LPVOID) -> HWND;

        pub fn RegisterClassW(lpWndClass : *const WNDCLASS) -> ATOM;
    }
}

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
