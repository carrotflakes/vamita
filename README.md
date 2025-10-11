# Vamita

A tiny Vampire Survivors-style arena built with [Bevy](https://bevyengine.org/) and Rust. Dodge endless waves of enemies, auto-fire projectiles, and chase a high score in a minimalist 2D arena.

## Features

- **Twin-stick style movement** using WASD or arrow keys
- **Auto-firing projectiles** that prioritise the nearest target
- **Scaling enemy waves** spawning from the arena edges
- **Compact HUD** showing your current score and remaining health

## Getting Started

### Prerequisites

- Rust toolchain (via [`rustup`](https://rustup.rs/))
- A recent version of `cargo`

### Run the game

```bash
cargo run --target x86_64-pc-windows-msvc
```

## Controls

| Input | Action |
|-------|--------|
| <kbd>W</kbd> / <kbd>↑</kbd> | Move up |
| <kbd>S</kbd> / <kbd>↓</kbd> | Move down |
| <kbd>A</kbd> / <kbd>←</kbd> | Move left |
| <kbd>D</kbd> / <kbd>→</kbd> | Move right |

Projectiles fire automatically at regular intervals; survive as long as you can.

## Next Steps

- Power-up drops that shift projectile patterns
- More enemy archetypes with unique movement
- Difficulty ramp based on elapsed time and score
