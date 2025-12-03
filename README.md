# Interaction Particule

## Requirement

* Install **rust** [here](https://www.rust-lang.org/tools/install).

* Install necessary lib for gtk4 :

```bash
# For Ubuntu 22.04+
$ sudo apt install libgtk-4-dev
# For Fedora
$ sudo dnf install gtk4-devel
```

## Project

Interactive particle simulation written in Rust using `gtk4` for rendering.

- **What it is:** A small simulation where colored particles (red/blue/green/yellow)
	interact according to configurable pairwise rules. Each rule controls the
	strength (`g`) and effective distance (`d`) of the interaction between two
	color groups.
- **Why it's useful:** Handy for experimenting with emergent behavior, flocking,
	attraction/repulsion systems and tuning simple physics-like interactions.

You can configure interaction rules in `rules.json` (project root). The project
includes a sample `rules.json` and a demo video showing the simulation in
action.

Example `rules.json`:

```json
[
	{"group1":"red","group2":"red","g":-1.0,"d":8.0},
	{"group1":"red","group2":"blue","g":1.0,"d":10.0}
]
```
Demo video

![demo animation](docs/demo.gif)

Quick start

```bash
# install dependencies and run
cargo run
```
