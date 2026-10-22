fn transpose_matrix(input: &str) -> Option<Vec<Vec<f64>>> {
    let matrix: Vec<Vec<f64>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let cols = matrix.first()?.len();

    Some(
        (0..cols)
            .map(|c| matrix.iter().map(|row| row[c]).collect())
            .collect(),
    )
}

fn main() {
    let input = "1 2 3\n4 5 6\n";
    println!("{:?}", transpose_matrix(input)); // Some([[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]])
    println!("{:?}", transpose_matrix("")); // None
}
