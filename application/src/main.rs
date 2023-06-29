use glfw::{self, glfwInit};

fn main() {
    unsafe {
        // Initialize GLFW
        if glfwInit() == -1 {
            println!("Failed to initialize GLFW!");
        };
    }
}
