pub mod array_generator {
    use rand::Rng;
    use rand::prelude::ThreadRng;
    use itertools::Itertools;

    pub(crate) fn generate_array(array_len: u32, min: i32, max: i32) -> Vec<i32> {
        let mut rng: ThreadRng= rand::thread_rng();
        let generated_array: Vec<i32> = (0..array_len).map(|_| rng.gen_range(min..max)).collect();
        generated_array.into_iter().unique().collect()
    }
}