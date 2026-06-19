#[derive(Debug, Copy, Clone)]
pub struct Terminal {
    columns: f64, // x-axis
    lines: f64,   // y-axis
    frame_rate: f64,
}

impl Terminal {
    pub fn new() -> Terminal {
        match term_size::dimensions() {
            Some((columns, lines)) => Terminal {
                columns: columns as f64,
                lines: lines as f64,
                frame_rate: 60.0,
            },
            None => {
                panic!("Terminal Size not found");
            }
        }
    }

    pub fn columns(&self) -> f64 {
        self.columns
    }
    pub fn lines(&self) -> f64 {
        self.lines
    }
    pub fn frame_rate(&self) -> f64 {
        self.frame_rate
    }

    pub fn set_lines(&mut self, lines: f64) -> &mut Self {
        self.lines = lines;
        self
    }
    pub fn set_columns(&mut self, columns: f64) -> &mut Self {
        self.columns = columns;
        self
    }
    pub fn set_frame_rate(&mut self, frame_rate: f64) -> &mut Self {
        self
    }
}
