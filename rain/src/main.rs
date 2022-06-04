use terminal_size::{Width, Height, terminal_size};

fn main() {
    println!("Hello, world!");
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        println!("Your terminal is {} cols wide and {} lines tall", w, h);
    } else {
        println!("Unable to get terminal size");
    }
}
