use std::io::stdout;
extern crate termion;

pub struct Bar {
    pub current_partial: usize,
    pub total: usize,
    last_bytes: usize,
}
impl Bar {
    pub fn new(total: usize) -> Bar {
        Bar {
            current_partial: 0,
            total: total,
            last_bytes: 0,
        }
    }
}

impl Bar {
    pub fn update(&mut self, mut newProgress: usize) {
        self.current_partial += newProgress;
        let percent = (self.current_partial as f32 / self.total as f32) * 100.0;
        let mut display = String::from("[");
        let itter = percent.floor() as usize / 2 as usize;
        for i in 0..itter {
            if (i % 2 == 0){
                display.push('X');
            }
        }
        for i in itter..100 as usize {
            if i % 2 == 0 {
                display.push(' ');
            }
        }
        display.push_str(format!("] {:2}%", percent).as_str());
        let mut bytes = display.as_bytes();
        let bytes_len = bytes.len();
        let mut stdout = stdout();
        print!("{}{}{}", termion::clear::CurrentLine, termion::cursor::Right(self.last_bytes as u16), display);
        
        self.last_bytes = display.len();
    }
}