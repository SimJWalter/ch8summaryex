use rand::Rng;
use std::collections::HashMap;

pub(crate) struct Ex1 {
    ints: Vec<i32>,
}

impl Ex1 {
    pub fn new(qu: i32) -> Self {
        let mut intvec: Vec<i32> = Vec::new();
        for _ in 0..qu {
            intvec.push(rand::thread_rng().gen_range(1..26));
        }
        intvec.sort_unstable();

        Self { ints: intvec }
    }

    pub fn len(&self) -> usize {
        self.ints.len()
    }

    pub fn mean(&self) -> f32 {
        self.ints.iter().sum::<i32>() as f32 / self.ints.len() as f32
    }

    pub fn median(&self) -> i32 {
        self.ints[self.ints.len() / 2]
    }

    pub fn mode(&self) -> i32 {
        let mut occurs = HashMap::new();

        for key in &self.ints {
            *occurs.entry(key).or_insert(0) += 1;
        }

        let g = occurs.into_iter();
        let h = g.max_by_key(|&(_, count)| count);
        let mut i = h.map(|(key, _)| key);
        i.expect("unable to max zero count value list");
        let ret = *(i.take().unwrap());

        ret
    }
}
