# DOGFIGHT

![DOGFIGHT](assets/image/DOGFIGHT.png)

## Introduction
Welcome to **DOGFIGHT**, a thrilling 3D flight simulation and exploration game built with the Bevy engine. In **DOGFIGHT**, players control an aircraft, explore dynamic worlds, and engage in intense aerial combat. The game features multiple flight modes, realistic physics, and an immersive third-person camera.

> **Note**: To play the game, you must place all game assets in the same directory as the executable.

---

## Features

### 1. **Flight Modes**
- **Arcade Mode**: Simplified controls for quick action and fun.
- **Realistic Mode**: Advanced flight physics for a challenging experience.
- Toggle modes by pressing the **Spacebar**.

### 2. **Third-Person Camera**
Implemented in `camera.rs`, the camera system offers:
- Dynamic aiming.
- Adjustable zoom levels.
- Smooth transitions and offsets for an immersive experience.

### 3. **Player Mechanics**
Defined in `player.rs`, the player features:
- Realistic movement with acceleration and deceleration.
- Interactive flight dynamics based on keyboard input and cursor position.
- Customizable 3D player model (`Player.glb`).

### 4. **World Environment**
Built with `world.rs`, the environment includes:
- A detailed 3D map (`map.glb`).
- Dynamic lighting for visually stunning gameplay.

### 5. **Physics Integration**
Physics are powered by the `bevy_rapier3d` plugin, ensuring realistic movement and collisions.

### 6. **Performance Monitoring**
The game integrates the `iyes_perf_ui` plugin for real-time FPS tracking and diagnostics.

---

## File Structure

```plaintext
DOGFIGHT/
├── src/
│   ├── camera.rs          # Third-person camera system.
│   ├── main.rs            # Game entry point.
│   ├── player.rs          # Player controls and movement.
│   └── world.rs           # World environment setup.
├── assets/
│   ├── image/DOGFIGHT.png # Project cover image.
│   ├── map.glb            # 3D map for the game world.
│   └── Player.glb         # 3D player model.
└── README.md              # Project documentation.
## Prerequisites

- **Rust** (latest stable version)
- **Bevy Engine** (`bevy` crate)
- Additional dependencies:
  - `bevy_third_person_camera`
  - `bevy_rapier3d`
  - `iyes_perf_ui`

---

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/DOGFIGHT.git
   cd DOGFIGHT

## Controls

- **W**: Increase speed
- **S**: Decrease speed
- **A**: Pitch up
- **Q**: Pitch down
- **Mouse**: Adjust aircraft direction based on cursor position
- **Spacebar**: Toggle between Arcade and Realistic flight modes

---

## Contribution

Contributions are welcome! If you'd like to improve **DOGFIGHT**, fork the repository and submit a pull request.

---

## License

This project is licensed under the **MIT License**.

---

## Acknowledgments

- A special thanks to the Bevy community for their tools and guidance.

---
