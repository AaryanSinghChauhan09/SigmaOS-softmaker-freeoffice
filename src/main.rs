//! SigmaOS-softmaker-freeoffice — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_softmaker_freeoffice_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-softmaker-freeoffice: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-softmaker-freeoffice: ok");
}
