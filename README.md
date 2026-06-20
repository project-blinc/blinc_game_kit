# blinc_game_kit

Game / 3D-asset toolkit for [Blinc](https://github.com/project-blinc/Blinc).

Merges the former [`blinc_gltf`](https://github.com/project-blinc/blinc_gltf)
(loader) + [`blinc_skeleton`](https://github.com/project-blinc/blinc_skeleton)
(runtime poser) into a single crate so consumers carry one git pin
instead of two and feature gating lives in one place.

Re-exposes both API surfaces behind top-level modules so the original
`blinc_gltf::Foo` / `blinc_skeleton::Bar` mental model carries over
verbatim:

```rust
use blinc_game_kit::gltf::load_glb;
use blinc_game_kit::skeleton::{Pose, Player};
use blinc_canvas_kit::SceneKit3D;

let bytes = std::fs::read("DamagedHelmet.glb")?;
let scene = load_glb(&bytes)?;

// Static scene
let kit = SceneKit3D::new("viewer");
let handles = scene.add_to(&kit);

// Skinned mesh with animation
let mut player = Player::new(&scene.skeletons[0], &scene.animations[0]);
player.set_looping(true);
loop {
    player.tick(dt);
    let skinning = player.skinning_matrices();
    // feed skinning into MeshData::skin on each primitive
}
```

## Modules

- **[`gltf`]** — glTF 2.0 loader. Parses `.glb` / `.gltf` via the
  upstream [`gltf`](https://crates.io/crates/gltf) crate and maps
  meshes / materials / nodes / skins / animations into `blinc_core`
  types (`MeshData`, `Vertex`, `Material`, `Skeleton`, `Bone`) plus a
  thin scene graph ready for `blinc_canvas_kit::SceneKit3D`.
- **[`skeleton`]** — runtime poser. Evaluates `GltfAnimation` channels
  each frame, composes joint hierarchies, emits GPU-ready skinning
  matrices that feed into `blinc_core::draw::SkinningData`. Also ships
  IK solvers (`solve_fabrik`, `solve_two_bone`), a state-machine
  driver for clip blending (`StateMachine`), and rotation densification
  for linear-interpolating samplers (`densify_rotation_channels`).

## What's mapped

| glTF concept | Blinc type |
|---|---|
| Primitives (position / normal / UV0 / color / tangent / joints / weights / indices) | `MeshData` / `Vertex` |
| `pbrMetallicRoughness` factors + all five texture slots | `Material` |
| `alphaMode` (OPAQUE / MASK / BLEND) | `AlphaMode` |
| `KHR_materials_unlit` | `Material::unlit` |
| Nodes (TRS or matrix, parent/child links) | `gltf::GltfNode` + `gltf::NodeTransform` |
| Skins (joints + inverse bind matrices) | `Skeleton` / `Bone` in `gltf::GltfSkeleton` |
| Animations (channels + samplers + interpolation) | `gltf::GltfAnimation` (data only) |

## Features

- `platform-assets` — enables `gltf::load_asset()` for cross-platform
  asset loading via the global `blinc_platform::assets` loader. Off by
  default so pure-desktop callers keep depending on `std::fs` via
  `gltf::load_path` without pulling in the platform crate.
- `bc-encode` — opt-in runtime BC1 / BC3 / BC4 encoding of decoded
  glTF textures. Adds ~50-150 ms per 2K × 2K texture to async load,
  but cuts GPU VRAM ~4-8× for material textures.

## Migrating from blinc_gltf + blinc_skeleton

```diff
-use blinc_gltf::{load_glb, GltfScene};
-use blinc_skeleton::{Pose, Player};
+use blinc_game_kit::gltf::{load_glb, GltfScene};
+use blinc_game_kit::skeleton::{Pose, Player};
```

In `Cargo.toml`:

```diff
-blinc_gltf = { git = "...", rev = "...", features = ["platform-assets", "bc-encode"] }
-blinc_skeleton = { git = "...", rev = "..." }
+blinc_game_kit = { git = "...", rev = "...", features = ["platform-assets", "bc-encode"] }
```
