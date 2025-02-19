mod algo;

pub use algo::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_should_work() {}

    #[test]
    fn algo_hash_should_work() {
        let algo = Algo::new(AlgoType::Blake3);
        let hash = algo.hash("hello");
        assert_eq!(
            hash,
            "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f"
        );
    }
}
