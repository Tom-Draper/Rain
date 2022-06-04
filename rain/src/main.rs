use crossterm_cursor::cursor;
use rand::Rng;
use terminal_size::{terminal_size, Height, Width};
use std::collections::VecDeque;


fn get_terminal_size() -> (u16, u16) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        return (w, h);
    } else {
        return (0, 0);
    }
}

fn create_raindrop(w: u16, h: u16) -> Raindrop {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(0..w);
    let y = rng.gen_range(1..3);
    let finish_y = rng.gen_range(h - 2..h - 1);

    return Raindrop { x, y, finish_y };
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Debug)]
pub struct Raindrop {
    x: u16,
    y: u16,
    finish_y: u16,
}

fn raining() {
    let mut cursor = cursor();
    let (w, h) = get_terminal_size();
    let mut raindrops: Vec<Raindrop> = Vec::new();

    for _row in 1..=1000 {
        let raindrop = create_raindrop(w, h);
        raindrops.push(raindrop);

        let mut index: usize = 0;
        let mut finished_raindrops: VecDeque<usize> = VecDeque::new();
        for r in raindrops.iter_mut() {
            // Remove old raindrop
            cursor.goto(r.x, r.y);
            print!(" ");
            if r.y < r.finish_y {
                // Place moved drop
                r.y = r.y + 1;
                cursor.goto(r.x, r.y);
                print!("|");
                cursor.hide();
            } else {
                finished_raindrops.push_front(index);
            }
            index += 1;
        }

        for i in finished_raindrops.iter() {
            raindrops.remove(*i);
        }
    }
}

fn main() {
    clear_screen();
    raining();
}
