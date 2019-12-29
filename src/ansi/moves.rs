pub fn up_delete(n: u64) -> String {
    let x = format!("\x1b[{}K", n);
    x.to_string()
}
