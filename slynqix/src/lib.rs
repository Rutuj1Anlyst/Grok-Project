 // slynqix/src/lib.rs
pub mod components;
pub mod hooks;
pub mod lib;

pub use components::*;
pub use hooks::*;
pub use lib::*;
// slynqix/src/lib.rs
pub mod components;
pub mod hooks;
pub mod lib;

pub use components::*;
pub use hooks::*;
pub use lib::*;

// Define utility functions (equivalent to "@/lib/utils")
pub mod utils {
    pub fn cn(classes: Vec<&str>) -> String {
        classes.join(" ")
    }
}