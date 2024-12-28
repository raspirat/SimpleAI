use colored::*;




pub fn message_constructor(heading: &str, message: &str) -> String {
    format!("[{}] {}", heading, message)
}

pub fn error_message_constructor(message: &str) -> String {
    message_constructor(&"Error".red().bold(), message)
}
pub fn warning_message_constructor(message: &str) -> String {
    message_constructor(&"Warning".yellow().bold(), message)
}

pub fn info_message_constructor(message: &str) -> String {
    message_constructor(&"Info".green().bold(), message)
}

pub fn render_warning(message: &str, e: anyhow::Error) {
    eprintln!("{}", warning_message_constructor(message));
    e.chain()
        .for_each(|cause| eprintln!("caused by: {}", cause));
}
