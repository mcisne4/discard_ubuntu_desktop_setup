pub struct Log {
    pub crate_idx: u8,
    pub mod_idx: u8,
    pub mod_path: String,
}
impl Log {
    pub fn new(crate_idx: u8, mod_idx: u8, mod_path: &str) -> Self {
        Self {
            crate_idx,
            mod_idx,
            mod_path: mod_path.to_owned(),
        }
    }

    pub fn log(log_type: u8, idx: u8, content: String) {
        //
    }
}

enum Test2 {
    CM001 { log: u8 },
}

enum Test {
    CM001,
}
impl Test {
    pub fn log(&self) {
        let x = match self {
            Self::CM001 => {
                Log {
                    crate_idx: 2,
                    mod_idx: 4,
                    mod_path: "26".to_owned(),
                };
                impl Log {
                    pub fn don() {}
                }
            }
        };
    }
}
// Logs::CM102::L102()
