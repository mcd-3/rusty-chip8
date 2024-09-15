use std::path::PathBuf;

pub fn read_rom_data(file_path: PathBuf) -> Vec<u8> {
    // println!("Opening file: {}", &file_path);

    // Test code to read rom data
    match std::fs::read(file_path) {
        Ok(bytes) => {
            println!("Loading into memory... Ok!");
            bytes
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("please run again with appropriate permissions.");
            }
            panic!("{}", e);
        }
    }
}