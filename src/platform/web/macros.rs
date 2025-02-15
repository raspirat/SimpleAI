#[macro_export]
macro_rules! web_platform_window {
    ($CURRENT_PAGE_NAME:expr) => {
        pub mod platform {
            use super::*;
            use crate::platform::window::Window;

            /// Creates a new window for the $CURRENT_PAGE_NAME page.
            ///
            /// # Returns
            /// A `Window` instance configured with the current page name.
            pub fn window() -> Window {
                // Create a new window using the current page name as the route.
                Window::new($CURRENT_PAGE_NAME)
            }
        }
    };
}


