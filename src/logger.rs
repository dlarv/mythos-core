static mut UTIL_ID: &str = "MYTHOS";
#[macro_export]
macro_rules! printinfo {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        println!("{} (Info): {}", get_id(), res);
    }}
}
#[macro_export]
macro_rules! printwarn {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        eprintln!("{} (Warning): {}", get_id(), res);
    }}
}
#[macro_export]
macro_rules! printerror {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        eprintln!("{} (Error): {}", get_id(), res);
    }}
}
#[macro_export]
macro_rules! printfatal {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        panic!("{} (Fatal Error): {}", get_id(), res);
    }}
}

pub fn set_id(util_id: &'static str) {
    unsafe {
        UTIL_ID = util_id;
    }
}
pub fn get_id() -> String {
    unsafe {
        return UTIL_ID.to_string();
    }
}
