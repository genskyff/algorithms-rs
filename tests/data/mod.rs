use rand::seq::SliceRandom;
use rand::{distributions::Uniform, Rng};

#[derive(Debug)]
pub(crate) struct TestData {
    pub unsorted: Vec<Vec<i32>>,
    pub sorted: Vec<Vec<i32>>,
}

impl TestData {
    const COUNT: usize = 10;

    pub(crate) fn new() -> Self {
        let mut rng = rand::thread_rng();
        let value_range = Uniform::new_inclusive(-100, 100);

        let mut unsorted = Vec::new();
        let mut sorted = Vec::new();

        unsorted.push(vec![]);
        unsorted.push(vec![rng.sample(&value_range)]);
        unsorted.push(vec![rng.sample(&value_range), rng.sample(&value_range)]);

        for vec in unsorted.iter_mut() {
            let mut sorted_vec = vec.clone();
            sorted_vec.sort();
            sorted.push(sorted_vec);
        }

        for _ in 0..Self::COUNT {
            let len = rng.gen_range(3..=20);
            let mut vec = Vec::new();

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
                vec.push(rng.gen_range(-100..0));
            } else {
                vec.push(rng.gen_range(1..=100));
            }

            vec.shuffle(&mut rng);
            unsorted.push(vec.clone());

            vec.sort();
            sorted.push(vec);
        }

        TestData { unsorted, sorted }
    }
}
