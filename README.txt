# Fire Red - Text Adventure Engine (Rust)

Welcome to the Fire Red text adventure project! This is a simple game engine written in Rust designed to create text-based adventures, inspired by classic RPGs.

## Description

This engine allows players to navigate through different game locations, interact with elements within those locations, and engage in conversations. The game's content, including location details, dialogue, and navigation options, is loaded dynamically from external JSON files (`locations.json` and `conversations.json`).

## Features

*   **Location-Based Navigation:** Explore different areas defined in `src/locations/locations.json`.
*   **Interactive Choices:** Make decisions based on presented options to navigate between locations, trigger interactions, or start conversations.
*   **Conversation System:** Engage in multi-line dialogues loaded from `src/locations/conversations.json`.
*   **Data-Driven Content:** Game world details (locations, descriptions, art, conversations, choices) are stored in JSON files, making it easier to modify and expand the game without changing the core code.
*   **State Persistence:** The engine keeps track of the player's current location using the `save` module.
*   **ASCII Art Support:** Displays ASCII art associated with locations and conversations for enhanced immersion.
*   **Modular Design:** Separates concerns for locations, saving, and potentially other functionalities (`func` module).

## Project Structure (Key Files)

*   `src/main.rs`: (Assumed) The main entry point for the application.
*   `src/locations/mod.rs`: Core logic for loading, displaying, and handling interactions within locations and conversations.
*   `src/locations/locations.json`: JSON file defining game locations, their properties (name, description, art, category), and player options (choices leading to other locations, interactions, or conversations).
*   `src/locations/conversations.json`: JSON file defining conversations, including associated art, dialogue lines, and the next location/interaction after the conversation ends.
*   `src/save.rs`: Handles saving and potentially loading game state, such as the player's current location.
*   `src/func.rs`: (Assumed) Contains helper or utility functions used across the project.

## Dependencies

This project relies on the following Rust crates:

*   `serde`: For data serialization and deserialization.
*   `serde_json`: Specifically for handling JSON data format.

These are typically managed via the `Cargo.toml` file.

## How to Run

1.  **Prerequisites:**
    *   Ensure you have the Rust toolchain (including `rustc` and `cargo`) installed. If not, visit https://www.rust-lang.org/tools/install.
    *   Make sure the `src/locations/locations.json` and `src/locations/conversations.json` files exist and are correctly formatted according to the structures defined in `src/locations/mod.rs`.

2.  **Navigate to the project directory:**
    ```bash
    cd /home/samarinara/Documents/Code/fire_red/
    ```

3.  **Build the project:**
    ```bash
    cargo build
    ```

4.  **Run the project:**
    ```bash
    cargo run
    ```

Follow the on-screen prompts to play the game!