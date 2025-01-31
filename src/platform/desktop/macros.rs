#[macro_export]
macro_rules! desktop_platform_window {
    ($CONFIG:expr) => {
        #[cfg(feature = "desktop")]
        pub mod platform {
            use crate::pages::start::*;

            use dioxus::desktop::{Config, LogicalSize, WindowBuilder, WindowCloseBehaviour};
            fn config() -> Config {
                $CONFIG
            }

            use crate::platform::window::Window;
            pub fn window() -> Window {
                Window { app, config }
            }
        }
    };
}