//! SigmaOS-servicemanager3 — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_servicemanager3_lib::{Component, ComponentLifecycle};

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-servicemanager3: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-servicemanager3: ok");
}
