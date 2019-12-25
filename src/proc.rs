use crate::arg;

pub fn run() {
    let _config = arg::parse_defaults();
    let arguments = arg::parse_arguments();
}
