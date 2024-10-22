🚀 Mars Rover Command Center 🌍
===============================

Welcome to the **Mars Rover Command Center**, where you're in control of exploring the Martian surface! Using the command-line interface, you can launch rovers, give them commands, and guide them across the rugged terrain of Mars. Will they make it, or will they get lost in the Martian abyss? 🌌

🌟 Features
-----------

-   **Multiple Rovers**: Launch multiple rovers and guide each with custom commands.
-   **Customizable Grid**: Define your own Mars grid boundaries to explore.
-   **Rover Movements**: Rotate and move forward with intuitive commands.
-   **Interactive Commands**: Enter a sequence of movements (L, R, F) to see where your rover ends up.
-   **Lost Rovers**: If a rover moves off the grid, it is marked as lost---but you can always launch a new one!
-   **Flexible Input**: Coordinates can be entered with or without parentheses for easier user experience.

🚧 How It Works
---------------

The world of Mars is modeled as a grid of size `m x n`. You can deploy multiple rovers, each with an initial position `(x, y)` and direction (`N`, `E`, `S`, `W`).

### Rover Commands

-   `L`: Rotate left by 90 degrees.
-   `R`: Rotate right by 90 degrees.
-   `F`: Move forward one space.

If a rover moves beyond the grid, it is lost, but its last known position is saved.

🛠️ Setup
---------

1.  **Install Rust** (if you don't have it installed):

    `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2.  **Clone the repository**:

    `git clone https://github.com/yourusername/mars-rover.git
    cd mars-rover`

3.  **Run the project**:

    `cargo run`

  🛠️ Setup with Docker 🐳
 --------------------------
 If you prefer to run the project using Docker, follow these steps:

1. **Build the Docker Image**
First, make sure you have Docker installed and running. Then, from the project directory, build the Docker image:

`docker build -t mars-rover .`

2. **Run the Project in Docker**
After building the image, you can run the project in an interactive terminal using:

`docker run -it mars-rover`

📝 Usage Instructions
---------------------

Once you run the program, you'll enter the Mars Rover Command Center. Follow the prompts to explore Mars with your rovers:

1.  **Set the Mars Grid Size**: You'll first be asked to define the boundaries of Mars. The grid size should be entered as two numbers separated by a space, like so:

    `5 5`

2.  **Launch a Rover**: Next, you'll be asked to launch a rover by entering its initial state. You can input the rover's coordinates and direction with or without parentheses:

    `0, 2, N`

    Or:

    `(0, 2, N)`

3.  **Enter Commands**: Once the rover is deployed, give it commands to rotate or move:

    `LFFR`

4.  **Launch Another Rover or Exit**: After the commands are executed, you can:

    -   Continue commanding the same rover.
    -   Launch a new rover with the command `"new"`.
    -   Exit the program with the command `"exit"`.



🔧 Program Details
------------------

The main components of the program are:

### Rover Struct

The `Robot` struct represents the rover, with attributes such as:

-   `x`, `y`: Position coordinates.
-   `direction`: Current facing direction (`N`, `E`, `S`, `W`).
-   `lost`: A flag to indicate if the rover is lost.

### Commands

-   **L**: Rotate the rover 90 degrees to the left.
-   **R**: Rotate the rover 90 degrees to the right.
-   **F**: Move the rover one step forward in the current direction. If the rover moves out of bounds, it will be marked as "lost".
-   Special commands:
    -   `"new"`: Launch a new rover.
    -   `"exit"`: Quit the game.
    -   `"I"`: Provides detailed instrcutions.


🚨 Error Handling
-----------------

The program provides feedback for:

-   **Invalid grid sizes**: If you enter something that's not two numbers, it will prompt you to try again.
-   **Invalid rover positions**: If your input isn't a valid position (e.g., non-numeric coordinates), you'll be asked to correct it.
-   **Invalid commands**: Only `L`, `R`, and `F` are valid commands, and the program will alert you to any mistakes.

🌟 Future  Feature Improvements
----------------------
1. **Obstacle Detection & Avoidance**:

    -   **Description**: Add randomly or manually placed obstacles on the grid that rovers must avoid during exploration. Rovers should detect these obstacles in advance and either stop or reroute.
    -   **Implementation**: Introduce additional grid cells marked as obstacles. Modify the movement logic to prevent rovers from entering these cells. If a rover encounters an obstacle, it could prompt the user to reroute or auto-terminate the rover.
    -   **Challenge**: Incorporate a pathfinding algorithm (e.g., A* or Dijkstra) to handle more complex navigation scenarios.

    
2. **Save/Load Functionality**:

    -   **Description**: Enable saving the current grid state and rover positions so users can resume exploration later.
    -   **Implementation**: Serialize the current state of the grid, rovers, and any additional game data into a file. On startup, allow the user to load the saved state or start a new session.
    -   **Challenge**: Ensure smooth serialization and deserialization of the game state to prevent data corruption.
    
 3. **Enhanced UI/Graphical User Interface (GUI)**:

    -   **Description**: Develop a graphical interface to visualize the grid, rover movements, obstacles, and more. This can provide a richer and more immersive experience for users.
    -   **Implementation**: Use a GUI library like **Tauri** (for Rust) or consider transitioning to a game engine like **Bevy** for advanced graphics.
    -   **Challenge**: Rendering performance for large grids and managing real-time interactivity between the user and the GUI.-   
    
4. **Rover Battery & Resource Management**:

    -   **Description**: Add a battery or fuel system where rovers have limited energy for exploration. Users will need to strategize and make efficient use of the rover's resources.
    -   **Implementation**: Introduce a countdown system to simulate resource depletion. Allow users to recharge or refuel rovers at certain points on the grid.
    -   **Challenge**: Balancing resource management with exploration gameplay to maintain user engagement.

💻 Code Improvements
------------------

1. **Refactor to Make Code More Modular**

-  **Why**: Your code could be broken down into smaller, more manageable modules to increase readability and reusability.
-  **How**: Separate responsibilities into different files or modules, such as creating a module for rover movement, grid management, and input/output handling.
- **Example**:
<br>mod rover: Manages all rover-related logic (movement, rotation).
<br>mod grid: Manages the grid and bounds-checking.
<br>mod input: Manages all user inputs and validation.

2.  **Unit Tests**

- **Why**: Ensure code reliability by testing individual units of logic.
- **How**: Use Rust's built-in testing framework to add unit tests for rover movements, boundary checks, and user inputs.

3. **Implement Monitoring**

- **Why**: If this project were to scale, you'd want to monitor performance, errors, and uptime.
- **How**: Add a logging framework and consider integrating with a tool like Prometheus for metrics collection.
Use Grafana dashboards to visualize how  rovers are being deployed, errors encountered, and the overall uptime of the system.

4. **Add A Timeout**
- **Why**: Add a timeout limit for rover commands to avoid infinite loops or stuck conditions.
- **How**: Implement a timer that tracks the time spent executing rover commands. If the time exceeds a predefined limit (e.g., 10 seconds), stop command execution and notify the user that the rover took too long. This prevents unintended endless processing.
