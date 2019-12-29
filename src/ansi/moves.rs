pub const ERASE: &str = "\x1b[2K"; // Reset

pub fn up_delete(n: u64) -> String {
    let x = format!("\x1b[{}K", n);
    x.to_string()
}

pub fn pos_x(n: u64) -> String {
    let x = format!("\x1b[{}G", n);
    x.to_string()
}
