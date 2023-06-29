// CString represents an owned, C-compatible, nul-terminated string with no nul bytes in the middle.
use std::ffi::CString;
use std::ptr;

// Internal Crates
use glfw::{self, glfwInit, glfwVulkanSupported, glfwWindowHint, GLFW_CLIENT_API, GLFW_NO_API, glfwCreateWindow};

fn main() {
    unsafe {
        // Initialize GLFW
        if glfwInit() != 1 {
            panic!("Failed to initialize GLFW!")
        };

        // Check for Vulkan support
        if glfwVulkanSupported() != 1 {
            panic!("Vulkan not supported on current platform.")
        }

        // Create a Vulkan context window

        // We disable context creation because we will use Vulkan, not OpenGL.
        glfwWindowHint(GLFW_CLIENT_API as i32, GLFW_NO_API as i32);

        let window_title = CString::new("2D Rendering").expect("Failed to create CString");
        let window_handle = glfwCreateWindow(
            800,
            600,
            window_title.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut());
    }
}
