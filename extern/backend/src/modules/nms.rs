pub mod create;
pub mod delete;
pub mod modify;

fn check_name(name: String) -> bool {
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
