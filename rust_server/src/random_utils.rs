pub fn pretty_print_system_time() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let datetime = chrono::DateTime::<chrono::Local>::from(UNIX_EPOCH + duration);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
