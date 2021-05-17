#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]

mod config;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let cfg = match config::get_config() {
        Ok(cfg) => cfg,
        Err(_) => panic!("Couldn't get config"),
    };
    for e in cfg.entries {
        for tc in e.termination_codes {
            println!("{} {:?}", e.name, tc)
        }
    }

    let app = thrust::TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
