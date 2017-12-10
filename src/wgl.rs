
use win32;

pub enum GLRC {}
pub type HGLRC = *mut GLRC;

pub enum OpaqueFunction {}
pub type GLPROC = *mut OpaqueFunction;

#[link(name = "opengl32")]
extern {
    #[link_name = "wglCreateContext"]
    pub fn CreateContext(hdc : win32::HDC) -> HGLRC;

    #[link_name = "wglMakeCurrent"]
    pub fn MakeCurrent(hdc : win32::HDC, hglrc : HGLRC) -> win32::BOOL;

    #[link_name = "wglGetProcAddress"]
    pub fn GetProcAddress(lpszProc : win32::LPCWSTR) -> GLPROC;
}

pub type ChoosePixelFormatARB_t = unsafe extern fn(
    hdc : win32::HDC,
    piAttribIList : *const win32::INT,
    pfAttribFList : *const win32::FLOAT,
    nMaxFormats : win32::UINT,
    piFormats : *mut win32::INT,
    nNumFormats : *mut win32::UINT) -> win32::BOOL;

#[allow(unused_variables)]
unsafe extern fn ChoosePixelFormatARB_stub(
    hdc : win32::HDC,
    piAttribIList : *const win32::INT,
    pfAttribFList : *const win32::FLOAT,
    nMaxFormats : win32::UINT,
    piFormats : *mut win32::INT,
    nNumFormats : *mut win32::UINT) -> win32::BOOL
{
    return win32::FALSE;
}

pub static mut ChoosePixelFormatARB : ChoosePixelFormatARB_t = ChoosePixelFormatARB_stub;
