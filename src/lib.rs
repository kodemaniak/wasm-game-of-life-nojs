use crate::universe::*;
use std::{cell::RefCell, fmt, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod universe;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

fn canvas() -> web_sys::CanvasRenderingContext2d {
    let canvas = document().get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut universe = Universe::new();
    let cell_size = 10;
    let padding = 2;

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("game-of-life-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    canvas.set_height((cell_size + 1) * (universe.height() + 1) + padding * 2);
    canvas.set_width((cell_size) * (universe.width() + 1) + padding * 2);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        universe.tick();

        context.begin_path();
        context.set_stroke_style(&JsValue::from_str("#CCCCCC"));

        // Vertical lines.
        for i in 0..(universe.width() + 1) {
            context.move_to((i * (cell_size) + padding) as f64, padding as f64);
            context.line_to(
                (i * (cell_size) + padding) as f64,
                ((cell_size) * (universe.height()) + padding) as f64,
            );
        }

        // Horizontal lines.
        for i in 0..(universe.height() + 1) {
            context.move_to(padding as f64, (i * (cell_size) + padding) as f64);
            context.line_to(
                ((cell_size) * (universe.width()) + padding) as f64,
                (i * (cell_size) + padding) as f64,
            );
        }

        let cells = universe.get_cells();
        for row in 0..universe.height() {
            for column in 0..universe.width() {
                let idx = universe.get_index(row, column);
                let color = match cells[idx] {
                    Cell::Alive => JsValue::from_str("#111111"),
                    Cell::Dead => JsValue::from_str("#eeeeee"),
                };
                let x = column * cell_size + padding + 1;
                let y = row * cell_size + padding + 1;
                context.set_fill_style(&color);
                context.fill_rect(
                    x as f64,
                    y as f64,
                    (cell_size - 1) as f64,
                    (cell_size - 1) as f64,
                )
            }
        }

        context.stroke();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
