use gba_convert::bitmap;

bitmap!(name: "invaders8", path: "test_data/invader.png", depth: 8);
bitmap!(name: "invaders", path: "test_data/invader.png", depth: 16);

fn main() {
    println!("{}", INVADERS_WIDTH);
    println!("{}", INVADERS_HEIGHT);
}
