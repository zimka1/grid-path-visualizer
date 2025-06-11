# A\* Pathfinding Visualizer in Rust

An interactive grid-based visualizer for the A* (A-star) pathfinding algorithm.
You can draw walls, set the start and goal positions, and watch the algorithm step through the grid to find the shortest path — or indicate when no path is possible.

![Screen Recording 2025-06-11 at 21 31 39 (online-video-cutter com)](https://github.com/user-attachments/assets/6b0cff48-f6ff-43f9-b272-e5184ca79423)


---

## Features

* Interactive grid editor
* Wall placement via mouse clicks
* Start and Goal selection
* Visualized search (Visited nodes)
* Shortest path traced in real-time
* Reset the board after path is drawn
* Graceful handling when no path exists

---

## Controls

| Key / Mouse        | Action                             |
| ------------------ | ---------------------------------- |
| `W`                | Switch to **Wall** placement mode  |
| `S`                | Switch to **Start** placement mode |
| `G`                | Switch to **Goal** placement mode  |
| `Mouse Left Click` | Place/remove depending on mode     |
| `Space`            | Run A\* algorithm                  |
| `R`                | Reset board (clears path/visited)  |
| `Esc / X`          | Close the window                   |

Start and Goal **must** be set before pressing `Space`.

---

## Project Structure

```
src/
├── main.rs         # Application entry point, handles events
├── astar.rs        # A* algorithm logic with step-by-step rendering
├── grid.rs         # Drawing logic and pixel-level cell rendering
└── types.rs        # CellType, PlacementMode, and Cell definition
```

---

## How It Works

1. **Grid Initialization**: A 2D vector of `Cell`, each with a type.
2. **User Interaction**: Mouse clicks and keypresses update the grid.
3. **A\* Algorithm**:

   * Uses a binary heap (min-heap with Reverse wrapper).
   * Calculates heuristic (Manhattan distance).
   * Tracks visited nodes and reconstructs the shortest path.
4. **Rendering**:

   * The grid is redrawn on every significant state change.
   * Colors reflect cell state (wall, visited, path, etc.).

