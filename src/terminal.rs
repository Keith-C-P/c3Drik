use term_size;
pub struct Terminal {
    columns: f64, // x-axis
    lines: f64,   // y-axis
}

impl Terminal {
    pub fn new() -> Terminal {
        let dims = term_size::dimensions();
        match dims {
            Some((width, height)) => {
                println!("{:?}", dims);
                Terminal {
                    columns: width as f64,
                    lines: height as f64,
                }
            }
            None => panic!("Terminal Size Not Found"),
        }
    }

    pub fn columns(&self) -> f64 {
        self.columns
    }
    pub fn lines(&self) -> f64 {
        self.lines
    }
}
