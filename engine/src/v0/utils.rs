use rand::prelude::SliceRandom;
use rand::RngCore;

pub(crate) fn random_bool_vec(n: usize, pct_true: f32, rng: &mut dyn RngCore) -> Vec<bool> {
    let mut result = vec![false; n];

    let num_true = ((n as f32) * pct_true) as usize;
    for b in result.iter_mut().take(num_true) {
        *b = true;
    }

    result.shuffle(rng);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_random_bool_vec() {
        let mut rng = Box::new(ChaCha8Rng::seed_from_u64(10914));

        assert_eq!(random_bool_vec(0, 0.0, &mut rng), vec![false; 0]);
        assert_eq!(random_bool_vec(0, 1.0, &mut rng), vec![false; 0]);

        assert_eq!(random_bool_vec(2, 0.0, &mut rng), vec![false, false]);
        assert_eq!(random_bool_vec(2, 0.5, &mut rng), vec![true, false]);
        assert_eq!(random_bool_vec(2, 1.0, &mut rng), vec![true, true]);

        assert_eq!(
            random_bool_vec(5, 0.0, &mut rng),
            vec![false, false, false, false, false]
        );
        assert_eq!(
            random_bool_vec(5, 0.6, &mut rng),
            vec![false, true, false, true, true]
        );
        assert_eq!(
            random_bool_vec(5, 1.0, &mut rng),
            vec![true, true, true, true, true]
        );
    }

    quickcheck! {
        fn random_bool_vec_num_true(n: usize, pct_true: f32, rnd_seed: u64) -> TestResult {
            if pct_true < 0.0 || pct_true > 1.0 {
                return TestResult::discard();
            }

            let mut rng = Box::new(ChaCha8Rng::seed_from_u64(rnd_seed));

            let v = random_bool_vec(n, pct_true, &mut rng);
            let expected_num_true = ((n as f32) * pct_true) as usize;
            let num_true = v.iter().filter(|&x| *x).count();

            TestResult::from_bool(num_true == expected_num_true)
        }
    }
}
