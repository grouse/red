#![windows_subsystem = "windows"]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

extern crate libc;
extern crate widestring;

use std::ptr::null_mut;
use std::os::raw::c_void;
use std::ffi::CString;
use std::ffi::CStr;

mod win32;
#[macro_use]
mod wgl;
mod gl;

#[macro_use]
mod log;

pub fn check_gl_error()
{
    unsafe {
        let mut err = gl::GetError();
        while err != gl::NO_ERROR {
            log!(log::Type::Error, "OpenGL error: {}", err);
            err = gl::GetError();
        }
    }
}

pub unsafe extern fn gl_debug_callback(
    source    : gl::GLenum,
    dbg_type  : gl::GLenum,
    id        : gl::GLuint,
    severity  : gl::GLenum,
    length    : gl::GLsizei,
    message   : *const gl::GLchar,
    userParam : *const c_void)
{
    let cstr = CStr::from_ptr(message);
    log!(log::Type::Debug, "OpenGL debug message: {}", cstr.to_str().unwrap());
}

pub unsafe extern fn window_proc(
    hwnd   : win32::HWND,
    msg    : win32::UINT,
    wparam : win32::WPARAM,
    lparam : win32::LPARAM) -> win32::LRESULT
{
    match msg {
        win32::WM_QUIT => quit(),
        win32::WM_CLOSE => quit(),
        win32::WM_DESTROY => {
            win32::PostQuitMessage(0);
        }
        win32::WM_PAINT => {
            let mut ps : win32::PAINTSTRUCT = std::mem::uninitialized();
            let hdc = win32::BeginPaint(hwnd, &mut ps);

            win32::FillRect(
                hdc,
                &mut ps.rcPaint,
                (win32::COLOR_WINDOW+1) as win32::HBRUSH);

            win32::EndPaint(hwnd, &ps);
        }
        win32::WM_KEYDOWN => {
            match wparam {
                win32::VK_ESCAPE => quit(),
                _ => log!(log::Type::Warning, "unhandled key code: {}", wparam)
            }
        }
        _ => return win32::DefWindowProcW(hwnd, msg, wparam, lparam)
    }

    return 0;
}

fn quit()
{
    std::process::exit(0);
}


fn main()
{
    let class_name = win32::wide_string("red");

    let wc = win32::WNDCLASS {
        lpfnWndProc   : Some(window_proc),
        lpszClassName : class_name,
        ..Default::default()
    };

    let hinst : win32::HINSTANCE;
    let mut hwnd      : win32::HWND;
    let mut hdc       : win32::HDC;
    let mut glcontext : wgl::HGLRC;

    unsafe {
        hinst = win32::GetModuleHandleW(null_mut());

        win32::RegisterClassW(&wc);

        let dwstyle : u32 =
            win32::WS_TILED | win32::WS_VISIBLE |
            win32::WS_MINIMIZEBOX | win32::WS_MAXIMIZEBOX | win32::WS_SYSMENU;

        hwnd = win32::CreateWindowExW(
            0,
            class_name,
            class_name,
            dwstyle,
            0, 0,
            1280, 720,
            null_mut(),
            null_mut(),
            hinst,
            null_mut());

        hdc = win32::GetDC(hwnd);

        let pfd = win32::PIXELFORMATDESCRIPTOR {
            dwFlags : win32::PFD_DOUBLEBUFFER |
                      win32::PFD_DRAW_TO_WINDOW |
                      win32::PFD_SUPPORT_OPENGL,
            iPixelType   : win32::PFD_TYPE_RGBA,
            cColorBits   : 32,
            cDepthBits   : 0,
            cStencilBits : 0,
            iLayerType   : win32::PFD_MAIN_PLANE,
            ..Default::default()
        };

        let pf = win32::ChoosePixelFormat(hdc, &pfd);
        assert!(pf != 0);

        let result = win32::SetPixelFormat(hdc, pf, &pfd);
        assert!(result != win32::FALSE);

        glcontext = wgl::CreateContext(hdc);
        let result = wgl::MakeCurrent(hdc, glcontext);
        assert!(result != win32::FALSE);

        wgl::ChoosePixelFormatARB = wglGetProcAddress!(
            "wglChoosePixelFormatARB",
            wgl::ChoosePixelFormatARB_t);

        wgl::CreateContextAttribsARB = wglGetProcAddress!(
            "wglCreateContextAttribsARB",
            wgl::CreateContextAttribsARB_t);

        gl::DebugMessageCallback = wglGetProcAddress!(
            "glDebugMessageCallback",
            gl::DebugMessageCallback_t);

        let attributes = [
            wgl::DRAW_TO_WINDOW_ARB, gl::TRUE,
            wgl::SUPPORT_OPENGL_ARB, gl::TRUE,
            wgl::DOUBLE_BUFFER_ARB,  gl::TRUE,
            wgl::PIXEL_TYPE_ARB,     wgl::TYPE_RGBA_ARB,
            wgl::COLOR_BITS_ARB,     32,
            wgl::DEPTH_BITS_ARB,     0,
            wgl::STENCIL_BITS_ARB,   0,
            0,        //End
        ];

        let mut format   : i32         = std::mem::uninitialized();
        let mut nformats : win32::UINT = std::mem::uninitialized();
        let result = wgl::ChoosePixelFormatARB(
            hdc, &attributes[0], null_mut(), 1, &mut format, &mut nformats);
        assert!(result != win32::FALSE);

        if format != pf {
            wgl::DeleteContext(glcontext);
            win32::DestroyWindow(hwnd);

            hwnd = win32::CreateWindowExW(
                0,
                class_name,
                class_name,
                dwstyle,
                0, 0,
                1280, 720,
                null_mut(),
                null_mut(),
                hinst,
                null_mut());

            hdc = win32::GetDC(hwnd);

            let result = win32::SetPixelFormat(hdc, format, &pfd);
            assert!(result != win32::FALSE);

            let tmp_context = wgl::CreateContext(hdc);
            let result = wgl::MakeCurrent(hdc, tmp_context);
            assert!(result != win32::FALSE);

            let result = wgl::MakeCurrent(hdc, tmp_context);
            assert!(result != win32::FALSE);

            let attributes = [
                wgl::CONTEXT_MAJOR_VERSION_ARB, 4,
                wgl::CONTEXT_MINOR_VERSION_ARB, 3,
                wgl::CONTEXT_FLAGS_ARB, wgl::CONTEXT_DEBUG_BIT_ARB,
                wgl::CONTEXT_PROFILE_MASK_ARB, wgl::CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
                0
            ];

            glcontext = wgl::CreateContextAttribsARB(hdc, null_mut(), &attributes[0]);
            wgl::MakeCurrent(null_mut(), null_mut());
            wgl::DeleteContext(tmp_context);

            let result = wgl::MakeCurrent(hdc, glcontext);
            assert!(result != win32::FALSE);

            let mut major : i32 = std::mem::uninitialized();
            let mut minor : i32 = std::mem::uninitialized();

            gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
            gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);

            log!(log::Type::Debug, "OpenGL version {}.{}", major, minor);

            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(gl_debug_callback, null_mut());

            gl::ClearColor( 0.0, 0.0, 0.0, 0.0 );
        }

    }

    log!(log::Type::Debug, "hello world!");

    loop {
        let mut msg;
        unsafe {
            msg = std::mem::uninitialized();
            while win32::PeekMessageW(&mut msg,
                                      null_mut(), 0, 0,
                                      win32::PM_REMOVE) != 0
            {
                win32::TranslateMessage(&mut msg);
                win32::DispatchMessageW(&mut msg);
            }

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::LoadIdentity();

            gl::Color3f( 1.0, 0.0, 0.0 );

            gl::Begin(gl::TRIANGLES);
                gl::Vertex3f(-0.5, -0.5, 0.0);
                gl::Vertex3f( 0.0,  0.5, 0.0);
                gl::Vertex3f( 0.5, -0.5, 0.0);
            gl::End();

            check_gl_error();

            win32::SwapBuffers(hdc);
        }
    }
}
