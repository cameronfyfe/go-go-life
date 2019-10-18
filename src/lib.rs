use wasm_bindgen::prelude::*;
use web_sys::console;
mod cell;
mod canvas;
mod universe;


#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    // Game of Life objects
    let mut universe = universe::Universe::new(400, 400);
    let mut canvas = canvas::Canvas::new("board");
    // Initial Game State
    universe.cells[4][4] = cell::State::Alive;
    universe.cells[20][30] = cell::State::Alive;
    // Initial Colors
    canvas.set_board_color("#000000");
    canvas.set_cell_color("#FFFFFF");
    // Draw State
    canvas.draw_universe(&universe);

    // TODO: make timer based loop so this can run in browser
    loop {
        universe.cycle();
        canvas.draw_universe(&universe);
    }

    Ok(())
}
