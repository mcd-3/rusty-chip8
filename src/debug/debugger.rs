pub struct Debugger { }

impl Debugger {
    #[cfg(debug_assertions)]
    pub fn dprint(message: String, is_error: bool) {
        if is_error {
            eprintln!("{}", message);
        } else {
            println!("{}", message);
        }
    }

    #[cfg(debug_assertions)]
    pub fn dprint_opcode(message: String, op_code: u16) {
        println!("{}{:#06X}", message, op_code);
    }

    #[cfg(not(debug_assertions))]
    pub fn dprint(_message: String, _is_error: bool) {}
    #[cfg(not(debug_assertions))]
    pub fn dprint_opcode(_message: String, _op_code: u16) {}
}