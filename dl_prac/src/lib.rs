#[cfg(test)]

mod tests {
    use neuronika::rand;
    #[test]
    fn test_data() {
        let dim = (2, 2);
        let x = rand(dim);
        // test data() api, which could return immutable Ref<Tensor<>> type;
        assert_eq!(x.data().dim(), dim);
    }
}