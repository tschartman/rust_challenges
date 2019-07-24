use permutohedron::Heap;

fn main() {
    let mut c_dim = vec![182, 123, 141, 154, 173, 202, 194, 211, 173];
    let mut b_dim = vec![19, 21, 17, 20, 12, 20, 14, 22, 21];
    println!("{}", calculate_crate2(&mut c_dim, &mut b_dim));
}

fn calculate_crate2(c_dim: &mut Vec<usize>, b_dim: &mut Vec<usize>) -> usize {
    let heap = Heap::new(b_dim);
    let mut permutations = Vec::new();
    let mut boxes = Vec::new();

    for data in heap {
        permutations.push(data.clone());
    }

    for i in 0..permutations.len() {
        boxes.push(calculate_crate(c_dim, &mut permutations[i]));
    }

    boxes.iter().cloned().fold(0, usize::max)
}

fn calculate_crate(c_dim: &mut Vec<usize>, b_dim: &mut Vec<usize>) -> usize  {
    let mut product = Vec::new();
    for i in 0..c_dim.len() {
        product.push(find_max_box(c_dim[i], b_dim[i]))
    }
    product.iter().product()
}

fn find_max_box(upper: usize, lower: usize) -> usize {
    upper / lower
}