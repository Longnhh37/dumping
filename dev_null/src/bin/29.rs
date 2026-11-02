fn help_frog_against_snakes(map: &str) -> (usize, Vec<Vec<u8>>) {
    let cells = map.as_bytes();
    let n = cells.len();

    let mut paths: Vec<Vec<Vec<u8>>> = vec![Vec::new(); n];
    paths[0].push(Vec::new());

    for i in 1..n {
        if cells[i] == b'*' {
            continue;
        }
        for jump in 1..=3u8 {
            let Some(prev) = i.checked_sub(jump as usize) else {
                continue;
            };
            let extended: Vec<Vec<u8>> = paths[prev]
                .iter()
                .map(|seq| {
                    let mut next = seq.clone();
                    next.push(jump);
                    next
                })
                .collect();
            paths[i].extend(extended);
        }
    }

    let res = std::mem::take(&mut paths[n - 1]);
    (res.len(), res)
}

fn main() {
    for map in ["A00**0*00B", "A0000B", "A000*****B"] {
        let (total, seqs) = help_frog_against_snakes(map);
        println!("{map}: total = {total}, sequences = {seqs:?}");
    }
}
