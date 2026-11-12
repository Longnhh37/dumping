use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rust_generate_button_text(status: String) -> String {
    match status.to_uppercase().as_str() {
        "PENDING" => "done".to_string(),
        "DONE" => "delete".to_string(),
        _ => "an error has occured".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::rust_generate_button_text;

    #[test]
    fn generates_actions_for_known_statuses_case_insensitively() {
        assert_eq!(rust_generate_button_text("PENDING".into()), "done");
        assert_eq!(rust_generate_button_text("done".into()), "delete");
    }

    #[test]
    fn reports_unknown_status() {
        assert_eq!(
            rust_generate_button_text("blocked".into()),
            "an error has occured"
        );
    }
}
