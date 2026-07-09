pub(super) fn expand_glob(word: String) -> Vec<String> {
    if !word.contains(['*', '?']) {
        return vec![word];
    }

    match glob::glob(&word) {
        Ok(paths) => {
            let matches: Vec<_> = paths
                .filter_map(Result::ok)
                .map(|p| p.to_string_lossy().into_owned())
                .collect();

            if matches.is_empty() { vec![word] } else { matches }
        }

        Err(_) => vec![word],
    }
}
