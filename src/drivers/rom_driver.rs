use std::path::PathBuf;
use crate::debug::debugger::Debugger;

pub struct RomDriver {
    file_path: PathBuf
}

impl RomDriver {
    pub fn new(file_path: PathBuf) -> Result<Self, &'static str> {
        Ok(RomDriver {
            file_path
        })
    }

    /// Reads a ROM file into memory
    pub fn read_rom_data(&self) -> Result<Vec<u8>, std::io::Error> {
        if cfg!(debug_assertions) {
            let fp: String = self.file_path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap();
    
            let mut message: String = String::from("Opening file: ");
            message.push_str(&fp);
    
            Debugger::dprint(message, false);
        }
    
        // Test code to read rom data
        match std::fs::read(&self.file_path) {
            Ok(bytes) => {
                Debugger::dprint(String::from("Loading into memory... Ok!"), false);
                Ok(bytes)
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    Debugger::dprint(String::from("Please run again with appropriate permissions."), true);
                }
                Err(e)
            }
        }
    }
}
