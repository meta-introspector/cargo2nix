use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RustcOptions {
    pub unpretty_expanded: bool,
    // Add other rustc options here as needed
}

impl RustcOptions {
    pub fn new() -> Self {
        RustcOptions {
            unpretty_expanded: false,
        }
    }
}
