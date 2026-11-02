type Mv = (char, u8);

fn extend_move(prev: &[Vec<Mv>], mv: Mv) -> impl Iterator<Item = Vec<Mv>> + '_ {
    prev.iter().map(move |seq| {
        let mut next = seq.clone();
        next.push(mv);
        next
    })
}

fn help_frog_against_snakes_2d(grid: &[&str]) -> (usize, Vec<Vec<Mv>>) {
    let g: Vec<&[u8]> = grid.iter().map(|row| row.as_bytes()).collect();
    let (rows, cols) = (g.len(), g[0].len());

    let mut paths: Vec<Vec<Vec<Vec<Mv>>>> = vec![vec![Vec::new(); cols]; rows];
    paths[0][0].push(Vec::new());

    for r in 0..rows {
        for c in 0..cols {
            if (r, c) == (0, 0) || g[r][c] == b'*' {
                continue;
            }
            let mut here: Vec<Vec<Mv>> = Vec::new();
            for j in 1..=3u8 {
                let step = j as usize;
                // Arrive by jumping right from (r, c - j)
                if let Some(pc) = c.checked_sub(step) {
                    here.extend(extend_move(&paths[r][pc], ('R', j)));
                }
                // Arrive by jumping down from (r - j, c)
                if let Some(pr) = r.checked_sub(step) {
                    here.extend(extend_move(&paths[pr][c], ('D', j)));
                }
            }
            paths[r][c] = here;
        }
    }

    let result = std::mem::take(&mut paths[rows - 1][cols - 1]);
    (result.len(), result)
}

fn main() {
    let grid = ["A00", "0*0", "00B"];
    let (total, seqs) = help_frog_against_snakes_2d(&grid);
    println!("total = {total}");
    for s in &seqs {
        println!("{s:?}");
    }
}
