use win32;

pub enum GLRC {}
pub type HGLRC = *mut GLRC;

pub enum OpaqueFunction {}
pub type GLPROC = *mut OpaqueFunction;


#[link(name = "opengl32")]
extern {
    #[link_name = "wglCreateContext"]
    pub fn CreateContext(hdc : win32::HDC) -> HGLRC;

    #[link_name = "wglDeleteContext"]
    pub fn DeleteContext(hglrc: HGLRC) -> win32::BOOL;

    #[link_name = "wglMakeCurrent"]
    pub fn MakeCurrent(hdc : win32::HDC, hglrc : HGLRC) -> win32::BOOL;

    #[link_name = "wglGetProcAddress"]
    pub fn GetProcAddress(lpszProc : win32::LPCWSTR) -> GLPROC;
}





//// extension: WGL_ARB_pixel_format
pub static mut ChoosePixelFormatARB : ChoosePixelFormatARB_t = ChoosePixelFormatARB_stub;

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
    unimplemented!();
}

// attributes
pub const NUMBER_PIXEL_FORMATS_ARB    : i32 = 0x2000;
pub const DRAW_TO_WINDOW_ARB          : i32 = 0x2001;
pub const DRAW_TO_BITMAP_ARB          : i32 = 0x2002;
pub const ACCELERATION_ARB            : i32 = 0x2003;
pub const NEED_PALETTE_ARB            : i32 = 0x2004;
pub const NEED_SYSTEM_PALETTE_ARB     : i32 = 0x2005;
pub const SWAP_LAYER_BUFFERS_ARB      : i32 = 0x2006;
pub const SWAP_METHOD_ARB             : i32 = 0x2007;
pub const NUMBER_OVERLAYS_ARB         : i32 = 0x2008;
pub const NUMBER_UNDERLAYS_ARB        : i32 = 0x2009;
pub const TRANSPARENT_ARB             : i32 = 0x200A;
pub const TRANSPARENT_RED_VALUE_ARB   : i32 = 0x2037;
pub const TRANSPARENT_GREEN_VALUE_ARB : i32 = 0x2038;
pub const TRANSPARENT_BLUE_VALUE_ARB  : i32 = 0x2039;
pub const TRANSPARENT_ALPHA_VALUE_ARB : i32 = 0x203A;
pub const TRANSPARENT_INDEX_VALUE_ARB : i32 = 0x203B;
pub const SHARE_DEPTH_ARB             : i32 = 0x200C;
pub const SHARE_STENCIL_ARB           : i32 = 0x200D;
pub const SHARE_ACCUM_ARB             : i32 = 0x200E;
pub const SUPPORT_GDI_ARB             : i32 = 0x200F;
pub const SUPPORT_OPENGL_ARB          : i32 = 0x2010;
pub const DOUBLE_BUFFER_ARB           : i32 = 0x2011;
pub const STEREO_ARB                  : i32 = 0x2012;
pub const PIXEL_TYPE_ARB              : i32 = 0x2013;
pub const COLOR_BITS_ARB              : i32 = 0x2014;
pub const RED_BITS_ARB                : i32 = 0x2015;
pub const RED_SHIFT_ARB               : i32 = 0x2016;
pub const GREEN_BITS_ARB              : i32 = 0x2017;
pub const GREEN_SHIFT_ARB             : i32 = 0x2018;
pub const BLUE_BITS_ARB               : i32 = 0x2019;
pub const BLUE_SHIFT_ARB              : i32 = 0x201A;
pub const ALPHA_BITS_ARB              : i32 = 0x201B;
pub const ALPHA_SHIFT_ARB             : i32 = 0x201C;
pub const ACCUM_BITS_ARB              : i32 = 0x201D;
pub const ACCUM_RED_BITS_ARB          : i32 = 0x201E;
pub const ACCUM_GREEN_BITS_ARB        : i32 = 0x201F;
pub const ACCUM_BLUE_BITS_ARB         : i32 = 0x2020;
pub const ACCUM_ALPHA_BITS_ARB        : i32 = 0x2021;
pub const DEPTH_BITS_ARB              : i32 = 0x2022;
pub const STENCIL_BITS_ARB            : i32 = 0x2023;
pub const AUX_BUFFERS_ARB             : i32 = 0x2024;

// attribute values
pub const NO_ACCELERATION_ARB      : i32 = 0x2025;
pub const GENERIC_ACCELERATION_ARB : i32 = 0x2026;
pub const FULL_ACCELERATION_ARB    : i32 = 0x2027;
pub const SWAP_EXCHANGE_ARB        : i32 = 0x2028;
pub const SWAP_COPY_ARB            : i32 = 0x2029;
pub const SWAP_UNDEFINED_ARB       : i32 = 0x202A;
pub const TYPE_RGBA_ARB            : i32 = 0x202B;
pub const TYPE_COLORINDEX_ARB      : i32 = 0x202C;


#[macro_export]
macro_rules! wglGetProcAddress {
    ($n:expr, $t:ty) => {
        std::mem::transmute::<wgl::GLPROC, $t>(wgl::GetProcAddress(win32::wide_string($n)));
    }
}

