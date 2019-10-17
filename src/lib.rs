use wasm_bindgen::prelude::*;
use web_sys::console;
mod draw;


#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    draw::draw_board();

    draw::draw_cells(&vec![
        draw::CellCoord{ x: 15, y: 15},
        draw::CellCoord{ x: 20, y: 20},
        draw::CellCoord{ x: 25, y: 25}
    ]);

    Ok(())
}
