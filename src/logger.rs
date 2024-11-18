use std::{fs::{File, OpenOptions}, io::{BufWriter, Error, Write}, path::PathBuf};
use std::cell::RefCell;
use crate::dirs::{self, MythosDir};


#[macro_export]
macro_rules! printinfo {
    ($switch:ident,$($arg:tt)*) => {{
        if $switch {
            let res = std::fmt::format(format_args!($($arg)*));
            mythos_core::logger::info(&res);
            println!("{}", res);
        }
    }};
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        mythos_core::logger::info(&res);
        println!("{}", res);
    }};
    // ($switch:expr,$($arg:tt)*) => {{
    //     if $switch {
    //         let res = std::fmt::format(format_args!($($arg)*));
    //         mythos_core::logger::info(&res);
    //         println!("{}", res);
    //     }
    // }};
}

#[macro_export]
macro_rules! printwarn {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        mythos_core::logger::warn(&res);
        eprintln!("Warning: {}", res);
    }}
}
#[macro_export]
macro_rules! printerror {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        mythos_core::logger::error(&res);
        eprintln!("Error: {}", res);
    }}
}
#[macro_export]
macro_rules! printfatal {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        mythos_core::logger::fatal(&res);
        eprintln!("Fatal: {}", res);
        std::process::exit(1);
    }}
}
#[macro_export]
macro_rules! fatalmsg{
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        format!("Fatal: {}", res)
    }}
}

// Singlethreaded access to global logger.
thread_local!(static LOGGER: RefCell<Logger> = RefCell::new(Logger::new("MYTHOS").unwrap()));

/// Change the id assigned to the logger. Default is MYTHOS.
pub fn set_id(id: &str) -> Result<(), Error> {
    let logger = Logger::new(id)?;
    LOGGER.with(|log| *log.borrow_mut() = logger);
    return Ok(());
}

pub fn info(msg: &str) -> String {
    LOGGER.with(|logger| logger.borrow_mut().write(msg, LogLevel::Info))
}
pub fn warn(msg: &str) -> String {
    LOGGER.with(|logger| logger.borrow_mut().write(msg, LogLevel::Warn))
}
pub fn error(msg: &str) -> String {
    LOGGER.with(|logger| logger.borrow_mut().write(msg, LogLevel::Error))
}
pub fn fatal(msg: &str) -> String {
    LOGGER.with(|logger| logger.borrow_mut().write(msg, LogLevel::Fatal))
}


#[derive(Debug)]
pub enum LogLevel { Info, Warn, Error, Fatal }

/// Writes to log file.
struct Logger {
    id: String,
    writer: BufWriter<File>,
}

impl Logger {
    pub fn new(id: &str) -> Result<Logger, Error> {
        // Automatically make log directory, if dne.
        let path = dirs::make_dir(MythosDir::Log, &id.to_lowercase())?;

        // Debug and release versions should have different files.
        let file_name = if cfg!(debug_assertions) {
            PathBuf::from("debug.log")
        } else {
            PathBuf::from("log")
        };
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path.join(file_name))?;

        return Ok(Logger {
            id: id.to_string(),
            writer: BufWriter::new(file),
        });
    }
    pub fn write(&mut self, msg: &str, level: LogLevel) -> String {
        let timestamp = chrono::Local::now();
        let msg = format!("{timestamp} {level:#?}: {msg}\n");
        let _ = self.writer.write(&msg.clone().into_bytes());
        return msg.to_string();
    }
}

#[cfg(test)]
mod test {
    use crate as mythos_core;

    #[test]
    fn test_logger() {
        let _ = super::set_id("TEST");
        super::info("Test entry");
    }
    #[test]
    fn test_print_info() {
        printinfo!(true, "Do print {}.", "this");
        printinfo!(false, "Do not print {}.", "this");
        // printinfo!(1 == 1, "Do print this.");
        printinfo!("This be valid");
        printinfo!("This should be valid too {}.", 0);
        // assert!(false);
    }
}
