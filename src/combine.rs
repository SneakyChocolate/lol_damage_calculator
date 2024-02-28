pub fn select<T>(list: &Vec<T>, n: usize) -> Vec<Vec<&T>> {
    let refs: Vec<&T> = list.iter().map(|e| e).collect();

    select_helper(&refs, n)
}

fn select_helper<T: Clone>(refs: &Vec<T>, n: usize) -> Vec<Vec<T>> {
    if n == 0 {
        return vec![vec![]];
    }

    let mut result: Vec<Vec<T>> = vec![];

    for i in 0..=refs.len() - n {
        let current: T = refs[i].clone();
        let remaining: Vec<T> = refs[i + 1..].to_vec();
        let combinations: Vec<Vec<T>> = select_helper(&remaining, n - 1);

        for combination in combinations {
            let mut temp = vec![current.clone()];
            temp.extend(combination);
            result.push(temp);
        }
    }

    return result;
}

pub fn multiply<T>(a: &Vec<&Vec<T>>) -> Vec<Vec<T>> {
    let mut max_iter = 100;
    let mut result: Vec<Vec<T>> = vec![];
    let mut counts: Vec<[usize; 2]> = vec![];
    for ae in a {
        counts.push([ae.len(), ae.len()]);
    }
    'a: while max_iter > 0 {
        max_iter -= 1;
        let n: Vec<T> = vec![];
        println!("{:?}", counts);
        
        // TODO add combination to n
        for c in counts.iter().enumerate() {}

        // add n to result
        result.push(n);
        
        // decrease count
        // TODO add last count to loop
        let len = counts.len();
        for i in 0..len {
            use std::cmp::Ordering;
            let c = &mut counts[i];
            match c[0].cmp(&1) {
                Ordering::Greater => {
                    c[0] -= 1;
                    if counts.iter().all(|count| {count[0] == 1}) {
                        break 'a;
                    }
                    break;
                },
                _ => c[0] = c[1],
            }
        }
    }
    result
}

#[cfg(test)]
mod combine_tests {
    use crate::combine::multiply;

    #[test]
    fn mul() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];

        let c = vec![&a, &b];

        let t = multiply(&c);

        assert_eq!(
            t,
            vec![
                vec![1, 4],
                vec![1, 5],
                vec![1, 6],
                vec![2, 4],
                vec![2, 5],
                vec![2, 6],
                vec![3, 4],
                vec![3, 5],
                vec![3, 6],
            ]
        );
    }
}
