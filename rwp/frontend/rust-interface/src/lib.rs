use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rust_generate_button_text(status: &str) -> String {
    if status.eq_ignore_ascii_case("PENDING") {
        "done"
    } else if status.eq_ignore_ascii_case("DONE") {
        "delete"
    } else {
        "an error has occured"
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::rust_generate_button_text;

    #[test]
    fn generates_actions_for_known_statuses_case_insensitively() {
        assert_eq!(rust_generate_button_text("PENDING"), "done");
        assert_eq!(rust_generate_button_text("done"), "delete");
    }

    #[test]
    fn reports_unknown_status() {
        assert_eq!(rust_generate_button_text("blocked"), "an error has occured");
    }
}
