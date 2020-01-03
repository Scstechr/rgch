pub mod arrows {
    pub const RET: &str = "\u{21B3}"; // ↳
}

pub mod colors {
    pub const X: &str = "\x1b[m"; // Reset
    pub const S: &str = "\x1b[1m"; // Strengthen. Bold.
    pub const U: &str = "\x1b[4m"; // Underline.

    pub const W: &str = "\x1b[30m"; // Weak (almost gone)
    pub const F: &str = "\x1b[90m"; // Fade

    pub const R: &str = "\x1b[91m"; // Red
    pub const G: &str = "\x1b[92m"; // Green
    pub const Y: &str = "\x1b[93m"; // Yellow
    pub const B: &str = "\x1b[94m"; // Blue
    pub const C: &str = "\x1b[96m"; // Cyan

    pub const FR: &str = "\x1b[31m"; // Fade Red
    pub const FG: &str = "\x1b[32m"; // Fade Green
    pub const FZ: &str = "\x1b[33m"; // Brown
    pub const FB: &str = "\x1b[34m"; // Deep Blue
    pub const FC: &str = "\x1b[36m"; // Fade Cyan
    pub const FY: &str = "\x1b[33m"; // Fade Yellow

    pub const BR: &str = "\x1b[48m"; // White w/ R BG.
    pub const BW: &str = "\x1b[3m"; // White w/ R BG.
}

pub mod moves {
    pub const ERASE: &str = "\x1b[2K"; // Reset

    pub fn up_delete(n: u64) -> String {
        format!("\x1b[{}K", n)
    }

    pub fn pos_x(n: u64) -> String {
        format!("\x1b[{}G", n)
    }
}

pub mod others {
    pub const TAB: &str = "  "; // Tab alternative
    pub const ARS: &str = ">> "; // Something before commands etc.
    pub const SAVE: &str = "\u{2398}"; // Something before commands etc.
    pub const CIRC: &str = "\u{25CF}";
    pub const SQRE: &str = "\u{25AA}";
    pub const GEAR: &str = "\u{2699}";
}

pub mod seg {
    pub const SH: &str = "\u{2502}"; // │
    pub const SV: &str = "\u{2500}"; // │
}
