use rand::seq::SliceRandom;
use rand::{distr::Uniform, Rng};

#[derive(Debug, Clone)]
pub struct TestData {
    pub unsorted: Vec<Vec<i32>>,
    pub sorted: Vec<Vec<i32>>,
}

impl TestData {
    const COUNT: usize = 100;
    const LEN: usize = 100;
    const RANGE: i32 = 1000;

    pub fn new() -> Self {
        let mut rng = rand::rng();
        let value_range = Uniform::new_inclusive(-Self::RANGE, Self::RANGE).unwrap();

        let mut unsorted = vec![];
        let mut sorted = vec![];

        unsorted.push(vec![]);
        unsorted.push(vec![0]);
        unsorted.push(vec![5, -2]);
        unsorted.push(vec![7, 0, -3]);

        let mut v = rng
            .clone()
            .sample_iter(&value_range)
            .take(20)
            .collect::<Vec<i32>>();
        v.sort();
        let mut v2 = v.clone();
        v2.reverse();

        unsorted.push(v);
        unsorted.push(v2);

        for vec in unsorted.iter_mut() {
            let mut sorted_vec = vec.clone();
            sorted_vec.sort();
            sorted.push(sorted_vec);
        }

        for _ in 0..Self::COUNT {
            let mut vec = vec![];
            let len = rng.random_range(3..Self::LEN);

            let duplicate_value = rng.sample(&value_range);
            vec.push(duplicate_value);
            vec.push(duplicate_value);

            for _ in vec.len()..len {
                vec.push(rng.sample(&value_range));
            }

            if !vec.contains(&0) {
                vec.push(0);
            }

            if vec.iter().all(|&x| x >= 0) {
                vec.push(rng.random_range(-Self::RANGE..0));
            } else {
                vec.push(rng.random_range(1..=Self::RANGE));
            }

            vec.shuffle(&mut rng);
            unsorted.push(vec.clone());

            vec.sort();
            sorted.push(vec);
        }

        TestData { unsorted, sorted }
    }
}
