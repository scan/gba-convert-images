use gba_convert::bitmap;

bitmap!(name: "invaders", path: "test_data/invaders.png", depth: 16);

fn main() {
    println!("{}", INVADERS_WIDTH);
}
