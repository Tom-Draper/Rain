use rand::Rng;
use terminal_size::{Width, Height, terminal_size};


fn get_terminal_size() -> (u16, u16) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        return (w, h);
    } else {
        return (0, 0);
    }
}

fn raindrop(w: u16, h: u16) {
    println!("Terminal is {} cols wide and {} lines tall", w, h);

    let mut rng = rand::thread_rng();
    println!("Start: {}, {}", rng.gen_range(0..w), rng.gen_range(1..5));
    println!("End: {}, {}", rng.gen_range(0..w), rng.gen_range(h-5..h-1));
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    clear_screen();
    let (w, h) = get_terminal_size();
    raindrop(w, h);
}
