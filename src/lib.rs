//! Game / 3D-asset toolkit for Blinc.
//!
//! Bundles the two sibling crates `blinc_gltf` (glTF 2.0 loader, maps
//! meshes / materials / nodes / skins / animations into `blinc_core`
//! types) and `blinc_skeleton` (runtime poser that evaluates
//! animations and emits GPU-ready skinning matrices) into a single
//! package so consumers pull one git pin instead of two and feature
//! gating lives in one place.
//!
//! Re-exports live behind two top-level modules so the
//! `blinc_gltf::Foo` / `blinc_skeleton::Bar` mental model carries
//! over verbatim:
//!
//! ```ignore
//! use blinc_game_kit::gltf::load_glb;
//! use blinc_game_kit::skeleton::{Pose, Player};
//!
//! let scene = load_glb(&bytes)?;
//! let mut player = Player::new(&scene.skeletons[0], &scene.animations[0]);
//! ```
//!
//! See the per-module docs for the loader / poser surfaces.

pub mod gltf;
pub mod skeleton;
