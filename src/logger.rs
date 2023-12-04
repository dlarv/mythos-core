static mut UTIL_ID: &str = "MYTHOS";

#[macro_export]
macro_rules! printinfo {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        println!("{}: {}", get_logger_id(), res);
    }}
}

#[macro_export]
macro_rules! printwarn {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        eprintln!("{} (Warning): {}", get_logger_id(), res);
    }}
}
#[macro_export]
macro_rules! printerror {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        eprintln!("{} (Error): {}", get_logger_id(), res);
    }}
}
#[macro_export]
macro_rules! printfatal {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        eprintln!("{} (Fatal Error): {}", get_logger_id(), res);
        std::process::exit(1);
    }}
}
#[macro_export]
macro_rules! fatalmsg{
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        format!("{} (Fatal): {}", get_logger_id(), res)
    }}
}


pub fn set_logger_id(util_id: &'static str) {
    unsafe {
        UTIL_ID = util_id;
    }
}
pub fn get_logger_id() -> String {
    unsafe {
        return UTIL_ID.to_string();
    }
}
