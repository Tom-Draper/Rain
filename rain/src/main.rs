use crossterm_cursor::{cursor, TerminalCursor};
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
    let y = 0;
    let finish_y = rng.gen_range(h - 3..h - 1);

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

fn travelling() {
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen_range(0..6);
    if n == 0 {
        print!("|");
    } else if n == 1 {
        print!("│"); 
    } else if n == 2 {
        print!("╎");
    } else if n == 3 {
        print!("┊");
    } else if n == 4 {
        print!("┆");
    } else if n == 5 {
        print!("╵");
    } else if n == 6 {
        print!("╷");
    }
}

fn landing() {
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen_range(0..5);
    if n == 0 {
        print!("o");
    } else if n == 1 {
       print!("○"); 
    } else if n == 2 {
        print!("◉");
    } else if n == 3 {
        print!("◌");
    } else if n == 4 {
        print!("◯");
    } else if n == 5 {
        print!("⊙");
    }
}

fn raining(w: u16, h: u16) {
    let cursor: TerminalCursor = cursor();
    cursor.hide();

    let mut raindrops: Vec<Raindrop> = Vec::new();
    let mut index: usize = 0;
    let mut finished_raindrops: VecDeque<usize>;

    loop {
        raindrops.push(create_raindrop(w, h));
        raindrops.push(create_raindrop(w, h));
        
        index = 0;
        finished_raindrops = VecDeque::new();
        for r in raindrops.iter_mut() {
            // Remove old raindrop
            cursor.goto(r.x, r.y);
            print!(" ");
            if r.y < r.finish_y {
                // Place moved drop
                r.y += 1;
                cursor.goto(r.x, r.y);
                if r.y == r.finish_y {
                    landing();
                } else {
                    travelling();
                }
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
    let (w, h) = get_terminal_size();
    raining(w, h);
}
