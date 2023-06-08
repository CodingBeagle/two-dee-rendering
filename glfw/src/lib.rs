// I allow certain stylish rules otherwise warned against by Rust,
// other you will be spammed with warnings when compiling the bindings for this crate
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// I supply my own type for VkSurfaceKHR and VkInstance,
// Which I blocklisted in the bindgen generation.
pub type VkSurfaceKHR = u64;
pub type VkInstance = u64;

include!(concat!(env!("OUT_DIR"), "/glfw-bindings.rs"));