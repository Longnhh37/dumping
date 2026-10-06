fn main() {
    let a = [2, 1, 5, 6, 2, 3];
    println!("prev_smaller: {:?}", prev_smaller(&a));
    // [None, None, Some(1), Some(2), Some(1), Some(4)]
    println!("prev_larger:  {:?}", prev_larger(&a));
    // [None, Some(0), None, None, Some(3), Some(3)]
    println!("next_smaller: {:?}", next_smaller(&a));
    // [Some(1), None, Some(4), Some(4), None, None]
    println!("next_larger:  {:?}", next_larger(&a));
    // [Some(2), Some(2), Some(3), None, Some(5), None]
}

fn prev_smaller<T: Ord>(a: &[T]) -> Vec<Option<usize>> {
    let mut res = vec![None; a.len()];
    let mut stack: Vec<usize> = Vec::new();

    for (i, x) in a.iter().enumerate() {
        while stack.last().is_some_and(|&j| a[j] >= *x) {
            stack.pop();
        }
        res[i] = stack.last().copied();
        stack.push(i);
    }
    res
}

fn prev_larger<T: Ord>(a: &[T]) -> Vec<Option<usize>> {
    let mut res = vec![None; a.len()];
    let mut stack: Vec<usize> = Vec::new();

    for (i, x) in a.iter().enumerate() {
        while stack.last().is_some_and(|&j| a[j] <= *x) {
            stack.pop();
        }
        res[i] = stack.last().copied();
        stack.push(i);
    }
    res
}

fn next_smaller<T: Ord>(a: &[T]) -> Vec<Option<usize>> {
    let mut res = vec![None; a.len()];
    let mut stack: Vec<usize> = Vec::new();

    for (i, x) in a.iter().enumerate() {
        while let Some(&j) = stack.last() {
            if a[j] > *x {
                res[j] = Some(i);
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    res
}

fn next_larger<T: Ord>(a: &[T]) -> Vec<Option<usize>> {
    let mut res = vec![None; a.len()];
    let mut stack: Vec<usize> = Vec::new();

    for (i, x) in a.iter().enumerate() {
        while let Some(&j) = stack.last() {
            if a[j] < *x {
                res[j] = Some(i);
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    res
}
