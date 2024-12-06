# **Space Invaders in Rust** 🛸🚀

Welcome to the **Space Invaders** project! This is a clone of the classic arcade game, developed in **Rust** using the **Piston** graphics library. Get ready to pilot a spaceship, shoot bullets, and face waves of alien invaders.

---

## **Game Features**

- 🎮 **Smooth controls** to move the spaceship and fire bullets.
- 💥 **Collision system** for bullets, enemies, and the spaceship.
- 👾 **Enemies arranged in rows and columns**, moving and changing direction.
- 📏 Modular design: code organized into reusable components.
- ⚡ **Optimized and clean** with idiomatic Rust patterns.

---

## **Prerequisites**

To run the game, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/) (version 1.70+ recommended).
- Cargo, included with Rust.
- A compatible operating system (Windows, macOS, or Linux).

---

## **How to Run the Game**

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/space-invaders-rust.git
   cd space-invaders-rust
   ```

2. Build and run the game:

   ```bash
   cargo run
   ```

3. Enjoy the game! Use the controls described below to play.

---

## **Controls**

- **Move the spaceship**:
  - **Left**: Left arrow (`←`)
  - **Right**: Right arrow (`→`)
- **Fire bullets**: Spacebar (`Space`)

---

## **Game Rules**

1. Control your spaceship to avoid enemies and their movements.
2. Shoot the aliens before they reach the bottom edge of the screen.
3. If an enemy touches your spaceship or reaches the bottom, you lose the game!

---

## **Code Structure**

The project follows a modular design for easier development and scalability:

- **`main.rs`**: The program's entry point, initializes the graphics engine and manages the main game loop.
- **Modules**:
  - `objects`: Contains the main entities:
    - `starship`: Implements the player's spaceship.
    - `bullet`: Defines the bullets fired by the spaceship.
    - `enemy`: Manages the game's enemies.
  - `managers`: Handle groups of objects:
    - `BulletManager`: Manages active bullets.
    - `EnemyManager`: Controls enemy movement and logic.
  - `draw`: Utilities for rendering elements on the screen.

---

## **Future Improvements**

- 🌌 **Animations** for explosions and firing effects.
- 🎵 **Sound effects** for bullets and collisions.
- 🌟 **Power-ups**: Add bonuses like double shots or shields.
- 🏆 **Score system**: Track your performance.
- 🕹️ **Multiplayer mode**: Enable two players to compete or cooperate.

---

## **Contributions**

Contributions are welcome! If you’d like to contribute:

1. **Fork** the repository.
2. Create a new branch for your changes:
   ```bash
   git checkout -b my-new-feature
   ```
3. Submit a **pull request** with your improvements.

---

## **License**

This project is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute this software.

---

## **Screenshots**

_(Coming soon: showcase images of the game in action)_

---

Thank you for playing **Space Invaders in Rust**! 👾✨  
If you have questions or suggestions, feel free to reach out. 🚀
