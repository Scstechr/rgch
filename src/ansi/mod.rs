/// Unicodes for arrows
pub mod arrows {
    /// ↳
    pub const RET: &str = "\u{21B3}";
}

/// Unicodes for changing text-styles
pub mod colors {
    /// Reset
    pub const X: &str = "\x1b[m";
    /// Strengthen. Bold.
    pub const S: &str = "\x1b[1m";
    /// Underline.
    pub const U: &str = "\x1b[4m";

    /// Weak (almost gone)
    pub const W: &str = "\x1b[30m";
    /// Fade
    pub const F: &str = "\x1b[90m";

    /// Red
    pub const R: &str = "\x1b[91m";
    /// Green
    pub const G: &str = "\x1b[92m";
    /// Yellow
    pub const Y: &str = "\x1b[93m";
    /// Blue
    pub const B: &str = "\x1b[94m";
    /// Cyan
    pub const C: &str = "\x1b[96m";

    /// Fade Red
    pub const FR: &str = "\x1b[31m";
    /// Fade Green
    pub const FG: &str = "\x1b[32m";
    /// Brown
    pub const FZ: &str = "\x1b[33m";
    /// Deep Blue
    pub const FB: &str = "\x1b[34m";
    /// Fade Cyan
    pub const FC: &str = "\x1b[36m";
    /// Fade Yellow
    pub const FY: &str = "\x1b[33m";

    /// White w/ R BG.
    pub const BR: &str = "\x1b[48m";
    /// White w/ R BG.
    pub const BW: &str = "\x1b[3m";
}

/// Unicodes for moving cursors, erasing lines
pub mod moves {
    /// Erase line
    pub const ERASE: &str = "\x1b[2K";

    /// Up and delete `n` line(s)
    pub fn up_delete(n: u64) -> String {
        format!("\x1b[{}K", n)
    }

    /// Move to the exact `n` horizontal from the left edge to the right
    pub fn pos_x(n: u64) -> String {
        format!("\x1b[{}G", n)
    }
}

/// Unicodes for segments
pub mod seg {
    /// │
    pub const SH: &str = "\u{2502}";
    /// │
    pub const SV: &str = "\u{2500}";
}

/// Unicodes for other purposes
pub mod others {
    /// Tab alternative
    pub const TAB: &str = "  ";
    /// Something before commands etc.
    pub const ARS: &str = ">> ";
    /// Generic save icon (floppy disk)
    pub const SAVE: &str = "\x1b[96m\u{2398}\x1b[m";
    /// Circle symbol
    pub const CIRC: &str = "\u{25CF}";
    /// Square symbol
    pub const SQRE: &str = "\u{25AA}";
    /// Gear-like symbol
    pub const GEAR: &str = "\u{2699}";
}
