# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

A Rust port of Peter Shirley's *Ray Tracing in One Weekend* (RTIOW), worked through listing by listing (commit messages track the book's listing numbers, e.g. "listing 6"). Code is meant to follow the book's C++ structure and naming closely, adapted to Rust.

## Commands

- Build: `cargo build`
- Render: `cargo run` — writes `gradient.ppm` (plain-text P3 PPM) to the current working directory; scanline progress goes to stdout.
- Lint: `cargo clippy`
- Format: `cargo fmt`

There are no tests yet.

## Architecture

- **Math types come from `nalgebra`, not a hand-written `vec3`.** `src/rtweekend.rs` defines `Vec3 = nalgebra::Vector3<f64>` and `Point3 = nalgebra::Point3<f64>`; `src/color.rs` aliases `Color = Vec3`. When translating book code, map `vec3` operations to nalgebra equivalents (`unit_vector` → `.normalize()`, `dot(a, b)` → `a.dot(&b)`, `.x()` → `.x`). Because `Point3` and `Vec3` are distinct nalgebra types, `Point3 - Point3` yields a `Vec3` and `Point3 + Vec3` yields a `Point3`; type mismatches between the two are a common compile error.
- `src/ray.rs` — `Ray { orig: Point3, dir: Vec3 }` with `at(t)`.
- `src/color.rs` — `write_color` writes one PPM pixel line to a `BufWriter<File>`, scaling [0,1] to [0,255].
- `src/main.rs` — image/camera setup, `ray_color`, scene intersection (currently `hit_sphere`), and the render loop. In the book, camera and hittable logic later move into their own modules (`camera`, `hittable`, `sphere`, `interval`, ...); new modules are registered via `mod` in `main.rs`.

Edition is 2024.
