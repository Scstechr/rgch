pub const ERASE: &str = "\x1b[2K"; // Reset

pub fn up_delete(n: u64) -> String {
    format!("\x1b[{}K", n)
}

pub fn pos_x(n: u64) -> String {
    format!("\x1b[{}G", n)
}
