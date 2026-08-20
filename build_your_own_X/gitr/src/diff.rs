use crate::index::read_index;
use crate::object::{read_object, ObjectType};
use crate::status::get_status;
use anyhow::{Context, Result, ensure};
use similar::{ChangeTag, TextDiff};
use std::collections::HashMap;
use std::fs;

pub fn diff() -> Result<()> {
    let (changed, _, _) = get_status().context("failed to get status for diff")?;
    let entries = read_index().context("failed to read index entries for diff")?;
    let entries_by_path: HashMap<_, _> = entries.into_iter().map(|e| (e.path.clone(), e)).collect();

    let num_changed = changed.len();
    for (i, path) in changed.into_iter().enumerate() {
        let entry = entries_by_path
            .get(&path)
            .with_context(|| format!("file '{}' missing from index entries", path))?;

        let sha1_hex = hex::encode(entry.sha1);
        let (obj_type, data) = read_object(&sha1_hex)
            .with_context(|| format!("failed to read object blob '{}'", sha1_hex))?;

        ensure!(
            obj_type == ObjectType::Blob,
            "expected blob object for path '{}', got {:?}",
            path,
            obj_type
        );

        let index_text = String::from_utf8_lossy(&data);
        let working_bytes = fs::read(&path)
            .with_context(|| format!("failed to read workspace file '{}'", path))?;
        let working_text = String::from_utf8_lossy(&working_bytes);

        let diff_res = TextDiff::from_lines(&index_text, &working_text);

        println!("--- {} (index)", path);
        println!("+++ {} (working copy)", path);

        for hunk in diff_res.grouped_ops(3) {
            for op in hunk {
                for change in diff_res.iter_changes(&op) {
                    let sign = match change.tag() {
                        ChangeTag::Delete => "-",
                        ChangeTag::Insert => "+",
                        ChangeTag::Equal => " ",
                    };
                    print!("{}{}", sign, change);
                }
            }
        }

        if i < num_changed - 1 {
            println!("{}", "-".repeat(70));
        }
    }

    Ok(())
}
