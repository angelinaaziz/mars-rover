use std::io::{self, BufRead};

// Define directions
#[derive(Debug, Copy, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

// Define the robot's struct
#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
    lost: bool,
}

// Implement movement for the robot
impl Robot {
    fn new(x: i32, y: i32, direction: Direction) -> Self {
        Robot {
            x,
            y,
            direction,
            lost: false,
        }
    }

    fn rotate_left(&mut self) {
        self.direction = match self.direction {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        };
    }

    fn rotate_right(&mut self) {
        self.direction = match self.direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        };
    }

    fn move_forward(&mut self, max_x: i32, max_y: i32) {
        match self.direction {
            Direction::N => {
                if self.y + 1 > max_y {
                    self.lost = true;
                } else {
                    self.y += 1;
                }
            }
            Direction::E => {
                if self.x + 1 > max_x {
                    self.lost = true;
                } else {
                    self.x += 1;
                }
            }
            Direction::S => {
                if self.y - 1 < 0 {
                    self.lost = true;
                } else {
                    self.y -= 1;
                }
            }
            Direction::W => {
                if self.x - 1 < 0 {
                    self.lost = true;
                } else {
                    self.x -= 1;
                }
            }
        }
    }
}

// Main function to handle input/output
fn main() {
    println!("🚀🌍 Welcome to the Mars Rover Command Center! 🌌✨");
    println!();
    println!("You're in control of exploring the surface of Mars with rovers! 🌍");
    println!("Guide them carefully across the Martian landscape. Let's roll! 🎮");
    println!();
    println!("📄 You can press 'I' anytime for detailed instructions. Let's go!");
    println!();

    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Loop for input processing
    loop {
        // Ask for grid size
        println!("🛠️ First, let's set the boundaries of Mars! 🗺️");
        println!("Enter the grid size in the format 'm n' (e.g., 5 5) or press 'I' for instructions:");

        let grid_size = input.next().unwrap().unwrap();

        // Check if user entered 'I' for instructions
        if grid_size.trim().to_uppercase() == "I" {
            instructions();
            continue;
        }

        // Parse the grid size input
        let dims: Vec<i32> = match grid_size.split_whitespace().map(|x| x.parse()).collect::<Result<Vec<i32>, _>>() {
            Ok(dimensions) => dimensions,
            Err(_) => {
                println!("🚨 Invalid grid size format. Please use the format 'm n'.");
                continue;
            }
        };

        if dims.len() != 2 {
            println!("🚨 Invalid input. Please enter two numbers separated by a space (e.g., 5 5).");
            continue;
        }

        let max_x = dims[0];
        let max_y = dims[1];

        println!("🌍 Mars grid size set to {} by {}. Now, launch a rover by entering its position as 'x, y, direction' (e.g., 0, 2, N) or without brackets! 🚀", max_x, max_y);

        while let Some(Ok(line)) = input.next() {
            // Check for 'I' to show instructions
            if line.trim().to_uppercase() == "I" {
                instructions();
                continue;
            }

            // Parse robot position and direction (brackets optional)
            let cleaned_line = line.trim_matches(|c| c == '(' || c == ')').replace(", ", " ");
            let robot_data: Vec<&str> = cleaned_line.split_whitespace().collect();

            if robot_data.len() != 3 {
                if robot_data[0].to_lowercase() == "exit" {
                    println!("👋 Exiting the Mars Rover Command Center. Safe travels!");
                    std::process::exit(0); // Terminate the program
                } else {
                    println!("🚨 Invalid input for the rover's initial state. Please try again.");
                    continue;
                }
            }

            let x: i32 = match robot_data[0].parse() {
                Ok(val) => val,
                Err(_) => {
                    println!("🚨 Invalid input for x-coordinate. Please enter a valid number.");
                    continue;
                }
            };
            let y: i32 = match robot_data[1].parse() {
                Ok(val) => val,
                Err(_) => {
                    println!("🚨 Invalid input for y-coordinate. Please enter a valid number.");
                    continue;
                }
            };

            let direction = match robot_data[2].to_uppercase().as_str() {
                "N" => Direction::N,
                "E" => Direction::E,
                "S" => Direction::S,
                "W" => Direction::W,
                _ => {
                    println!("🚨 Invalid direction. Please enter one of N, E, S, or W.");
                    continue;
                }
            };

            let mut robot = Robot::new(x, y, direction);

            // Keep playing with the same rover until it's lost
            loop {
                // Parse the command sequence
                println!("🎮 Enter the command sequence for the rover (L = Left, R = Right, F = Forward) or type 'new' to launch a new rover, or 'exit' to quit:");

                if let Some(Ok(commands)) = input.next() {
                    let trimmed_input = commands.trim().to_uppercase();  // Convert input to uppercase

                    if trimmed_input == "NEW" {
                        println!("🛠️ Launching a new rover. Please enter its initial state as 'x, y, direction' (e.g., 0, 2, N):");
                        break; // Break the loop to start launching a new rover
                    } else if trimmed_input == "EXIT" {
                        println!("👋 Exiting the Mars Rover Command Center. Safe travels!");
                        std::process::exit(0); // Terminate the program
                    }

                    // Validate the commands before processing
                    let valid_commands = trimmed_input.chars().all(|c| c == 'L' || c == 'R' || c == 'F');

                    if !valid_commands {
                        println!("❌ Invalid command. Use only 'L', 'R', or 'F'. Please try again.");
                        continue; // Re-prompt for command sequence without resetting the program
                    }

                    // Process each command in the sequence
                    for command in trimmed_input.chars() {
                        if robot.lost {
                            break;
                        }
                        match command {
                            'L' => robot.rotate_left(),
                            'R' => robot.rotate_right(),
                            'F' => robot.move_forward(max_x, max_y),
                            _ => unreachable!(), // We've already validated the commands
                        }
                    }

                    if robot.lost {
                        println!("💥 Rover is LOST, last seen at position ({}, {}, {:?})", robot.x, robot.y, robot.direction);
                        println!("🛠️ You can launch a new rover by entering its initial state or type 'exit' to quit.");
                        break;
                    } else {
                        println!("✅ Rover final position: ({}, {}, {:?})", robot.x, robot.y, robot.direction);
                        println!("🎮 You can enter more commands for this rover or type 'new' to launch a new one, or 'exit' to quit.");
                    }
                }
            }

            println!("🛠️ You can launch another rover or press 'I' for instructions.");
        }
    }
}

// Instructions function
fn instructions() {
    println!("📝 Instructions:");
    println!("1. Set the grid size in the format 'm n' (e.g., 5 5).");
    println!("2. Define the rover's starting position as (x, y, direction) (e.g., (0, 2, N)).");
    println!("   - Directions: N (North), E (East), S (South), W (West).");
    println!("3. Enter a sequence of commands:");
    println!("   - 'L' to rotate the rover left.");
    println!("   - 'R' to rotate the rover right.");
    println!("   - 'F' to move the rover forward.");
    println!("4. If the rover moves off the grid, it will be marked as LOST.");
    println!("5. You can press 'I' at any time to see these instructions again.");
    println!();
}
