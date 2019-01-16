fn lis(seq: &[i32]) -> Vec<i32> {
    let mut lis_list = vec![vec![seq[0]]];
    let mut max_lis = lis_list[0].clone();
    for i in 1..seq.len() {
        lis_list.push(Vec::new());
        for j in 0..i {
            if seq[j] < seq[i] && lis_list[j].len() > lis_list[i].len() {
                lis_list[i] = lis_list[j].clone();
            }
        }
        lis_list[i].push(seq[i]);
        if lis_list[i].len() > max_lis.len() {
            max_lis = lis_list[i].clone();
        }
    }
    max_lis
}

fn main() {
    println!("{:?}", lis(&[0, 8, 4, 2, 7, 5, 3, 1, 4, 8, 10]));
}
