# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

A Rust port of Peter Shirley's *Ray Tracing in One Weekend* (RTIOW), worked through section by section; commits are named after the book section they finish (e.g. "section 9.5 done"). The whole first book (through chapter 14, the final scene) is implemented, and work has moved on to *Ray Tracing: The Next Week*: the BVH (bounding volume hierarchy) is done, but motion blur was skipped, so there is no ray time or moving-sphere code. Code follows the book's C++ structure and naming closely, adapted to Rust — see "Differences from the book" below.

## Commands

- Build: `cargo build --release` (always render with `--release`; debug builds are far slower)
- Render: `cargo run --release` — writes `gradient.ppm` (plain-text P3 PPM) to the current working directory; scanline progress goes to stdout. The final scene at book settings (1200px, 500 samples/pixel) takes on the order of 10+ minutes on 8 cores; for quick checks, temporarily lower `image_width`/`samples_per_pixel` in `main.rs`.
- Lint: `cargo clippy` (`AABB` keeps the book's name with `#[allow(clippy::upper_case_acronyms)]`)
- Format: `cargo fmt`

There are no tests.

## Architecture

- **Math types come from `nalgebra`, not a hand-written `vec3`.** `src/rtweekend.rs` defines `Vec3 = nalgebra::Vector3<f64>` and `Point3 = nalgebra::Point3<f64>`; `src/color.rs` aliases `Color = Vec3`. Map book `vec3` operations to nalgebra (`unit_vector` → `.normalize()`, `dot(a, b)` → `a.dot(&b)`, `length` → `.norm()`, `length_squared` → `.norm_squared()`, `cross` → `a.cross(&b)`, `.x()` → `.x`, color `*` color → `.component_mul(&c)`). `Point3` and `Vec3` are distinct types: `Point3 - Point3` yields a `Vec3` and `Point3 + Vec3` yields a `Point3`; mixing them up is a common compile error.
- **Rendering pipeline:** `main.rs` builds a `HittableList` world of `Sphere`s, wraps it with `BVHNode::from_list(world)`, configures a `Camera` through its public fields, and calls `cam.render(&world)`. `Camera::render` calls `initialize()` (derives image height, viewport, u/v/w basis, defocus disk), shoots `samples_per_pixel` jittered rays per pixel via `get_ray`, and `ray_color` recurses up to `max_depth` bounces, asking the hit object's material how to scatter.
- **Hittables and bounding boxes:** `src/hittable.rs` defines two traits: `Hittable` (`hit`) and `BBox` (`bounding_box() -> AABB`), with `BBox` as a supertrait of `Hittable` so a `dyn Hittable` can report its box. Every hittable implements both, in separate `impl` blocks. Each object computes its `AABB` (`src/aabb.rs`) once at construction and stores it in a `bbox` field; `HittableList::add` grows the list's box as objects are added.
- **BVH:** `src/bvh_node.rs` builds the tree by taking the bounding box of the objects, splitting along its longest axis, sorting on that axis, and recursing on each half; 1 or 2 objects are stored directly. `hit` tests the node's `AABB` first, then only accepts a right-child hit that is closer than the left one.
- **Materials:** `src/material.rs` has the `Material` trait plus `Lambertian`, `Metal` (with fuzz), and `Dielectric` (refraction + Schlick reflectance). Each `Sphere` holds an `Arc<dyn Material>`, and `HitRecord.mat` carries it back to the camera.
- New modules are registered via `mod` in `main.rs`.

## Differences from the book

- **Multithreaded rendering with `rayon`** (not in the book): `Camera::render` computes scanlines in parallel with `into_par_iter()`, collects them, then writes them in order. Because the world is shared across threads, `Hittable` and `Material` require `Send + Sync`, and materials are shared with `Arc` (where the book uses `shared_ptr`). Any new hittable or material must be thread-safe.
- **Random numbers come from the `rand` crate** (`rand::random::<f64>()` inside `random_double`) instead of C++ `rand()`; it is thread-local, so it's safe to call from rayon threads.
- **Output goes to `gradient.ppm`**, not stdout as in the book.
- **BVH ownership:** `BVHNode` children are `Arc<dyn Hittable>` (the book's `shared_ptr`) so a single-object node can point `left` and `right` at the same object. `HittableList` stores `Box`es, which `BVHNode::from_list` consumes and converts with `Arc::from`. `BVHNode::new` panics on an empty object list. The book's `pad_to_minimums` (needed for quads) is not implemented yet.
- **Separate `BBox` trait:** the book puts `bounding_box()` on `hittable` itself; here it lives in its own `BBox` trait, required by `Hittable`. Rust can't write `dyn Hittable + BBox`, so the supertrait is what lets a `dyn Hittable` expose its box.
- **`Option` instead of out-parameters:** `Hittable::hit` returns `Option<HitRecord>` (not `bool` + `hit_record&`), and `Material::scatter` returns `Option<(attenuation, scattered_ray)>`.
- **`vec3.h` helpers are free functions in `rtweekend.rs`** (`random_vec3`, `random_vec3_min_max`, `random_unit_vector`, `random_on_hemisphere`, `random_in_unit_disk`, `near_zero`, `reflect`, `refract`, `degrees_to_radians`), since methods can't be added to the nalgebra type aliases. The book's `infinity` and `pi` constants are not defined; use `f64::INFINITY` and `std::f64::consts::PI`.
- **`Camera::new()`** sets the book's defaults; settings are public fields assigned in `main.rs`, and derived values are private fields filled in by `initialize()`.
- Unused parts of the book's API (`HittableList::clear`, `Interval::contains`/`expand`/`UNIVERSE`, `AABB::new`/`UNIVERSE`, `random_on_hemisphere`) are kept with a narrow `#[allow(dead_code)]` so the build stays warning-free.

Edition is 2024.
