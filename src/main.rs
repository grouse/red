#![windows_subsystem = "windows"]

extern crate libc;
extern crate widestring;

use std::ptr::null_mut;
use std::mem::size_of;

mod win32;
mod opengl;

pub unsafe extern fn window_proc(
    hwnd   : win32::HWND,
    msg    : win32::UINT,
    wparam : win32::WPARAM,
    lparam : win32::LPARAM) -> win32::LRESULT
{
    match msg {
        win32::WM_QUIT => std::process::exit(0),
        win32::WM_CLOSE => std::process::exit(0),
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
        _ => return win32::DefWindowProcW(hwnd, msg, wparam, lparam)
    }

    return 0;
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
    let hwnd  : win32::HWND;

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

        let hdc = win32::GetDC(hwnd);

        let pfd = win32::PIXELFORMATDESCRIPTOR {
            dwFlags : win32::PFD_DOUBLEBUFFER |
                      win32::PFD_DRAW_TO_WINDOW |
                      win32::PFD_SUPPORT_OPENGL,
            iPixelType   : win32::PFD_TYPE_RGBA,
            cColorBits   : 32,
            cDepthBits   : 24,
            cStencilBits : 8,
            iLayerType   : win32::PFD_MAIN_PLANE,
            ..Default::default()
        };

        let pf = win32::ChoosePixelFormat(hdc, &pfd);
        assert!(pf != 0);

        let result = win32::SetPixelFormat(hdc, pf, &pfd);
        assert!(result == win32::TRUE);

        let hglrc = opengl::wglCreateContext(hdc);
        opengl::wglMakeCurrent(hdc, hglrc);

        opengl::wglChoosePixelFormatARB =
            std::mem::transmute::<
                opengl::GLPROC,
                opengl::wglChoosePixelFormatARB_t>(
                    opengl::wglGetProcAddress(
                        win32::wide_string("wglChoosePixelFormatARB")));

    }

    println!("hello world!");

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
        }
    }
}
