use win32;
use std::string::String;

#[derive(Debug)]
pub enum Type {
    Debug,
    Warning,
    Error
}

pub fn internal_log(t : Type, file : &str, line : u32, msg : String)
{
    let tstr = match t {
        Type::Debug => "debug",
        Type::Warning => "warning",
        Type::Error => "error",
        _ => "unknown"
    };

    let formatted = format!("{}:{}: {}: {}\n", file, line, tstr, msg);
    unsafe {
        win32::OutputDebugStringW(win32::wide_string(formatted.as_str()));
    }
}

#[macro_export]
macro_rules! log {
    ($type:expr, $($arg:tt)+) => ({
        ::log::internal_log($type, file!(), line!(), format!($($arg)+))
    })
}
