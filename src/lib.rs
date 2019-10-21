use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::console;
mod cell;
mod canvas;
mod universe;


#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn cancelInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    // Game of Life objects
    let universe: Rc<RefCell<universe::Universe>> =
        Rc::new(RefCell::new(universe::Universe::new(400/5, 400/5)));
    let canvas: Rc<RefCell<canvas::Canvas>> =
        Rc::new(RefCell::new(canvas::Canvas::new("board")));
    // Initial Colors
    canvas.borrow_mut().set_board_color("#000000");
    canvas.borrow_mut().set_cell_color("#FFFFFF");
    // Initial Game State
    universe.borrow_mut().add_glider(10, 10);
    universe.borrow_mut().add_box(30, 33);
    universe.borrow_mut().add_box(26, 33);
    universe.borrow_mut().add_box(12, 30);
    universe.borrow_mut().add_box(35, 40);
    universe.borrow_mut().add_box(40, 40);
    canvas.borrow_mut().draw_universe(&universe.borrow_mut());
    // Interval Callback
    {
        let universe = universe.clone();
        let canvas = canvas.clone();
        let closure = Closure::wrap(Box::new(move || {
            canvas.borrow_mut().draw_universe(&universe.borrow_mut());
            universe.borrow_mut().cycle();
        }) as Box<dyn FnMut()>);
        setInterval(&closure, 40);
        closure.forget();
    }

    Ok(())
}
