use std::path::PathBuf;
use crate::debug::debugger::Debugger;

pub fn read_rom_data(file_path: PathBuf) -> Vec<u8> {
    if cfg!(debug_assertions) {
        let fp: String = file_path
            .clone()
            .into_os_string()
            .into_string()
            .unwrap();

        let mut message: String = String::from("Opening file: ");
        message.push_str(&fp);

        Debugger::dprint(message, false);
    }

    // Test code to read rom data
    match std::fs::read(file_path) {
        Ok(bytes) => {
            Debugger::dprint(String::from("Loading into memory... Ok!"), false);
            bytes
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                Debugger::dprint(String::from("Please run again with appropriate permissions."), true);
            }
            panic!("{}", e);
        }
    }
}