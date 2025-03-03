pub mod create;
pub mod delete;
pub mod modify;
pub mod query;

fn check_name(name: String) -> bool {
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
