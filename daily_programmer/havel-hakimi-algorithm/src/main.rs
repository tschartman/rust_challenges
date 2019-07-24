fn main() {
    let mut vec = vec![5, 3, 0, 2, 6, 2, 0, 7, 2, 5];
    println!("{}", havel_hakimi(&mut vec));
}

fn havel_hakimi (set: &mut Vec<u64> ) -> bool {
   if  eliminate_zeros(set).is_empty() {
        true
   } else {
        descend_sort(set);
        let x = set[0];
        set.remove(0);
        if less_than_len(x, set) {
            false
        } else {
            front_elimination(x, set);
            havel_hakimi(set)
        }
   }
}

fn eliminate_zeros( set: &mut Vec<u64> ) -> &mut Vec<u64> {
    set.retain(|&x| x != 0);
    set
}

fn descend_sort(set: &mut Vec<u64> ) -> &mut Vec<u64> {
    set.sort_by( |a, b| b.cmp(a) );
    set
}

fn less_than_len(x: u64, set: &mut Vec<u64> ) -> bool {
    let is_less = (x as usize) > set.len();
    is_less
}

fn front_elimination(x: u64, set: &mut Vec<u64> ) -> &mut Vec<u64> {
    for i in 0..(x as usize) {
       set[i] = set[i] - 1; 
    }

    set
}