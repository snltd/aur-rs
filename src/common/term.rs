use terminal_size::{terminal_size, Width};

pub fn term_width() -> usize {
    let size = terminal_size();
    if let Some((Width(w), _)) = size {
        w as usize
    } else {
        80
    }
}
