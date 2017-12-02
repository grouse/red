
mod win32 {
    use std;
    use libc;
    use widestring::WideCString;

    type LPVOID    = *mut std::os::raw::c_void;
    type PVOID     = LPVOID;
    type HANDLE    = PVOID;
    type HWND      = HANDLE;
    type HMENU     = HANDLE;
    type HINSTANCE = HANDLE;
    type BOOL      = libc::c_int;
    type DWORD     = libc::c_uint;
    type UINT      = libc::c_uint;
    type INT       = libc::c_int;
    type LPCWSTR   = *const u16;
    type LPWSTR   =  *mut   u16;

    const WS_TILED   : u32 = 0x00000000;
    const WS_VISIBLE : u32 = 0x10000000;

    pub fn wide_string(s : &str) -> LPWSTR
    {
        return WideCString::from_str(s).unwrap().into_raw();
    }

    #[link(name = "user32")]
    extern "stdcall" {
        pub fn CreateWindowW(
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
            lpPara       : LPVOID) -> HWND;
    }
}
