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

🌟 Future Improvements
----------------------

-   **Obstacle Detection**: Add obstacles to the grid that rovers must avoid.
-   **Save/Load Functionality**: Save rover progress to resume exploration later.
-   **Enhanced UI**: Create a graphical user interface for a more immersive experience.

