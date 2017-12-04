use std;
use libc;
use widestring::WideCString;

pub const WS_TILED   : u32 = 0x00000000;
pub const WS_VISIBLE : u32 = 0x10000000;
pub const PM_REMOVE  : u32 = 0x0001;

pub const WM_DESTROY : u32 = 0x0002;
pub const WM_QUIT    : u32 = 0x0012;
pub const WM_PAINT   : u32 = 0x0F;

pub const CS_VREDRAW : u32 = 0x0001;
pub const CS_HREDRAW : u32 = 0x0002;
pub const CS_OWNDC   : u32 = 0x0020;


pub const COLOR_WINDOW : u32 = 5;

pub type VOID      = std::os::raw::c_void;
pub type LPVOID    = *mut std::os::raw::c_void;
pub type PVOID     = LPVOID;
pub type HANDLE    = PVOID;
pub type HWND      = HANDLE;
pub type HMENU     = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HDC       = HANDLE;
pub type HMODULE   = HANDLE;
pub type HICON     = HANDLE;
pub type HCURSOR   = HANDLE;
pub type HBRUSH    = HANDLE;

pub type WORD  = u16;
pub type DWORD = u32;

pub type BOOL = i32;
pub type BYTE = u8;
pub type UINT = u32;
pub type INT  = u32;
pub type LONG = u32;

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

#[repr(C)]
pub struct MSG {
    pub hwnd:    HWND,
    pub message: UINT,
    pub wParam:  WPARAM,
    pub lParam:  LPARAM,
    pub time:    DWORD,
    pub pt:      POINT
}
pub type LPMSG = *mut MSG;

#[repr(C)]
pub struct POINT {
    pub x : LONG,
    pub y : LONG
}

#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc         : HDC,
    pub fErase      : BOOL,
    pub rcPaint     : RECT,
    pub fRestore    : BOOL,
    pub fIncUpdate  : BOOL,
    pub rgbReserved : [BYTE;32]
}
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;

#[repr(C)]
pub struct RECT {
    left   : LONG,
    top    : LONG,
    right  : LONG,
    bottom : LONG
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
    pub fn GetModuleHandleW(lpModuleName : LPCWSTR) -> HMODULE;

    pub fn PeekMessageW(
        lpMsg         : LPMSG,
        hWnd          : HWND,
        wMsgFilterMin : UINT,
        wMsgFilterMax : UINT,
        wRemoveMsg    : UINT) -> BOOL;

   pub fn TranslateMessage(lpMsg : *const MSG) -> BOOL;
   pub fn DispatchMessageW(lpMsg : *const MSG) -> LRESULT;

   pub fn PostQuitMessage(nExitCode : INT) -> VOID;

   pub fn DefWindowProcW(
       hWnd   : HWND,
       Msg    : UINT,
       wParam : WPARAM,
       lParam : LPARAM) -> LRESULT;

   pub fn BeginPaint(hwnd : HWND, lpPaint : LPPAINTSTRUCT) -> HDC;
   pub fn EndPaint(hWnd : HWND, lpaint : *const PAINTSTRUCT) -> BOOL;
   pub fn FillRect(hDC : HDC, lprc : *const RECT, hbr : HBRUSH) -> INT;
}
