#![allow(dead_code)]

use std;
use std::os::raw::{c_char, c_void};
use libc;
use widestring::WideCString;

pub const WS_TILED       : u32 = 0x00000000;
pub const WS_VISIBLE     : u32 = 0x10000000;
pub const WS_MINIMIZEBOX : u32 = 0x00020000;
pub const WS_MAXIMIZEBOX : u32 = 0x00010000;
pub const WS_SYSMENU     : u32 = 0x00080000;

pub const PM_REMOVE : u32 = 0x0001;

pub const WM_DESTROY : u32 = 0x0002;
pub const WM_CLOSE   : u32 = 0x0010;
pub const WM_QUIT    : u32 = 0x0012;
pub const WM_KEYDOWN : u32 = 0x0100;
pub const WM_PAINT   : u32 = 0x0F;

// virtual key codes
pub const VK_ESCAPE : WPARAM = 0x1B;

pub const CS_VREDRAW : u32 = 0x0001;
pub const CS_HREDRAW : u32 = 0x0002;
pub const CS_OWNDC   : u32 = 0x0020;

pub const PFD_TYPE_RGBA       : u8 = 0;
pub const PFD_TYPE_COLORINDEX : u8 = 1;

pub const PFD_MAIN_PLANE : u8 = 0;

pub const PFD_DOUBLEBUFFER    : u32 = 0x00000001;
pub const PFD_DRAW_TO_WINDOW  : u32 = 0x00000004;
pub const PFD_SUPPORT_OPENGL  : u32 = 0x00000020;

pub const COLOR_WINDOW : u32 = 5;

pub type VOID      = c_void;
pub type LPVOID    = *mut c_void;
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
pub const TRUE  : i32 = 1;
pub const FALSE : i32 = 0;

pub type FLOAT = f32;

pub type BYTE = u8;
pub type UINT = u32;
pub type INT  = i32;
pub type LONG = u32;

pub type ATOM = WORD;

pub type LONG_PTR  = libc::intptr_t;
pub type ULONG_PTR = libc::uintptr_t;

pub type WPARAM = libc::uintptr_t;
pub type LPARAM = LONG_PTR;

pub type LRESULT = LONG_PTR;

pub type LPCWSTR = *const u16;
pub type LPCSTR  = *const c_char;
pub type LPWSTR  = *mut   u16;

pub enum OpaqueFunction {}
pub type PROC = *mut OpaqueFunction;

type WNDPROC = unsafe extern fn (
    hwnd   : HWND,
    uMsg   : UINT,
    wParam : WPARAM,
    lParam : LPARAM) -> LRESULT;

pub fn wide_string(s : &str) -> LPWSTR
{
    return WideCString::from_str(s).unwrap().into_raw();
}

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

#[repr(C)]
pub struct PIXELFORMATDESCRIPTOR {
  pub nSize           : WORD,
  pub nVersion        : WORD,
  pub dwFlags         : DWORD,
  pub iPixelType      : BYTE,
  pub cColorBits      : BYTE,
  pub cRedBits        : BYTE,
  pub cRedShift       : BYTE,
  pub cGreenBits      : BYTE,
  pub cGreenShift     : BYTE,
  pub cBlueBits       : BYTE,
  pub cBlueShift      : BYTE,
  pub cAlphaBits      : BYTE,
  pub cAlphaShift     : BYTE,
  pub cAccumBits      : BYTE,
  pub cAccumRedBits   : BYTE,
  pub cAccumGreenBits : BYTE,
  pub cAccumBlueBits  : BYTE,
  pub cAccumAlphaBits : BYTE,
  pub cDepthBits      : BYTE,
  pub cStencilBits    : BYTE,
  pub cAuxBuffers     : BYTE,
  pub iLayerType      : BYTE,
  pub bReserved       : BYTE,
  pub dwLayerMask     : DWORD,
  pub dwVisibleMask   : DWORD,
  pub dwDamageMask    : DWORD
}
pub type PPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;

impl Default for PIXELFORMATDESCRIPTOR {
    fn default() -> PIXELFORMATDESCRIPTOR {
        PIXELFORMATDESCRIPTOR {
            nSize           : std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as WORD,
            nVersion        : 1,
            dwFlags         : 0,
            iPixelType      : 0,
            cColorBits      : 0,
            cRedBits        : 0,
            cRedShift       : 0,
            cGreenBits      : 0,
            cGreenShift     : 0,
            cBlueBits       : 0,
            cBlueShift      : 0,
            cAlphaBits      : 0,
            cAlphaShift     : 0,
            cAccumBits      : 0,
            cAccumRedBits   : 0,
            cAccumGreenBits : 0,
            cAccumBlueBits  : 0,
            cAccumAlphaBits : 0,
            cDepthBits      : 0,
            cStencilBits    : 0,
            cAuxBuffers     : 0,
            iLayerType      : 0,
            bReserved       : 0,
            dwLayerMask     : 0,
            dwVisibleMask   : 0,
            dwDamageMask    : 0
        }
    }
}

#[link(name = "user32")]
extern  {
    pub fn OutputDebugStringW( lpOutputString : LPCWSTR) -> c_void;

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

    pub fn DestroyWindow(hWnd : HWND) -> BOOL;

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

   pub fn GetDC(hwnd : HWND) -> HDC;
}

#[link(name = "gdi32")]
extern {
   pub fn ChoosePixelFormat(hdc : HDC, ppfd : *const PIXELFORMATDESCRIPTOR) -> INT;
   pub fn SetPixelFormat(
       hdc : HDC,
       iPixelFormat : INT,
       ppfd : *const PIXELFORMATDESCRIPTOR) -> BOOL;
}
