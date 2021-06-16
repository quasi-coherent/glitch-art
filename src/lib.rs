pub mod cmds;
pub mod effects;

use std::fmt;

pub struct Canvas {
    pub row_min: u32,
    pub row_max: u32,
    pub col_min: u32,
    pub col_max: u32,
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "row_min: {}, row_max: {}, col_min: {}, col_max: {}",
            self.row_min, self.row_max, self.col_min, self.col_max
        )
    }
}
