#![crate_id = "raytrace_core"]
//#![deny(missing_doc)]
#![crate_type = "rlib"]
pub mod camera;
pub mod film;
pub mod geometry;
pub mod transform;
pub mod sampler;
pub mod sample;
pub mod shape;
pub mod spectrum;
pub mod aggregator;
pub mod filter;
pub mod primitive;
pub mod bsdf;
pub mod material;
