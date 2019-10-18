use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::cell;
use crate::universe;


pub struct Canvas {
    board_w: u16,
    board_h: u16,
    board_color: String,
    cell_w: u16,
    cell_h: u16,
    cell_color: String,
    ctx: web_sys::CanvasRenderingContext2d
}


fn get_canvas_ctx_by_id(id: &str) -> web_sys::CanvasRenderingContext2d {
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();
    let canvas = document.get_element_by_id(id)
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


impl Canvas {
    pub fn new(canvas_id: &str) -> Canvas {
        let ctx = get_canvas_ctx_by_id(canvas_id);
        Canvas {
            board_w: 400,
            board_h: 400,
            board_color: "#000000".to_string(),
            cell_w: 5,
            cell_h: 5,
            cell_color: "#FFFFFF".to_string(),
            ctx: ctx
        }
    }

    pub fn set_board_color(&mut self, color: &str) {
        self.board_color = color.to_string();
    }

    pub fn set_cell_color(&mut self, color: &str) {
        self.cell_color = color.to_string();
    }

    pub fn draw_universe(&self, universe: &universe::Universe) {
        self.draw_board();
        for i in 0..universe.cells.len() {
            for j in 0..universe.cells[i].len() {
                match universe.cells[i][j] {
                    cell::State::Alive => self.draw_cell((i as u16, j as u16)),
                    _ => {}
                }
            }
        }
    }

    pub fn draw_board(&self) {
        self.ctx.set_fill_style(&JsValue::from_str("#000000"));
        self.ctx.fill_rect(0.0, 0.0, self.board_w as f64, self.board_h as f64);
    }

    pub fn draw_cell(&self, c: (u16, u16)) {
        self.ctx.set_fill_style(&JsValue::from_str("#FFFFFF"));
        let (x, y) = c;
        self.ctx.fill_rect(
            x as f64,
            y as f64,
            self.cell_w as f64,
            self.cell_h as f64
        );
    }

    pub fn draw_cells(&self, cells: &Vec<(u16, u16)>) {
        for cell in cells {
            self.draw_cell(*cell);
        }
    }
}