use notan::draw::*;
use notan::prelude::*;
use snek::*;
#[notan_main]
fn main() -> Result<(), String> {
    let n_config = WindowConfig::new()
        .title("snake")
        .size(640, 480)
        .vsync(true)
        .resizable(false);
    notan::init_with(h_setup)
        .add_config(n_config)
        .add_config(DrawConfig)
        .draw(h_draw)
        .build()
}
