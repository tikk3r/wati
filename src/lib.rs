use wasm_bindgen::prelude::*;

use itertools::Itertools;

use web_sys::js_sys::Uint8Array;

mod func_plot;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

#[wasm_bindgen]
pub struct Chart {}

#[wasm_bindgen]
pub fn shared_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
impl Chart {
    // OLD (use-after-free): returned dangling pointer to Vec<f64>
    // #[allow(clippy::too_many_arguments)]
    // pub fn plot_interferometer_uvcoverage(
    //     dec: f64,
    //     nu: f64,
    //     n_chan: u32,
    //     phi: f64,
    //     duration: f64,
    //     n_times: usize,
    //     array: &str,
    //     antenna_mask: &Uint8Array,
    // ) -> *const usize {
    //     let mask: Vec<u8> = antenna_mask.to_vec();
    //     let (u, v) =
    //         func_plot::draw_uvcoverage(dec, nu, n_chan, phi, duration, n_times, array, mask);
    //     let merged: Vec<f64> = u.into_iter().interleave(v).collect();
    //     merged.as_ptr() as *const usize
    // }

    #[allow(clippy::too_many_arguments)]
    pub fn plot_interferometer_uvcoverage(
        dec: f64,
        nu: f64,
        n_chan: u32,
        phi: f64,
        duration: f64,
        n_times: usize,
        array: &str,
        antenna_mask: &Uint8Array,
    ) -> Vec<f64> {
        let mask: Vec<u8> = antenna_mask.to_vec();
        let (u, v) =
            func_plot::draw_uvcoverage(dec, nu, n_chan, phi, duration, n_times, array, mask);
        u.into_iter().interleave(v).collect()
    }
}
