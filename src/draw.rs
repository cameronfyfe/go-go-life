use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


const board_w: u16 = 400;
const board_h: u16 = 400;
const cell_w: u16 = 5;
const cell_h: u16 = 5;


pub struct CellCoord {
    pub x: u16,
    pub y: u16,
}


// TODO: determine best way to handle global reference like this
fn get_ctx() -> web_sys::CanvasRenderingContext2d {
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();
    let canvas = document.get_element_by_id("board")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    ctx
}


pub fn draw_board() {
    let ctx = get_ctx();
    ctx.set_fill_style(&JsValue::from_str("#000000"));
    ctx.fill_rect(0.0, 0.0, board_w as f64, board_h as f64);
}


pub fn draw_cell(c: &CellCoord) {
    let ctx = get_ctx();
    ctx.set_fill_style(&JsValue::from_str("#FFFFFF"));
    ctx.fill_rect(
        (c.x * cell_w) as f64,
        (c.y * cell_h) as f64,
        cell_w as f64,
        cell_h as f64
    );
}


pub fn draw_cells(cells: &[CellCoord]) {
    for cell in cells {
        draw_cell(cell);
    }
}
