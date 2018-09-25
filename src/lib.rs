#[derive(Clone, Debug, PartialEq)]
struct Bucketizer {
    buckets: Vec<Bucket>,
}

type Bucket = (Option<f64>, Option<f64>, f64);

impl Bucketizer {
    fn new() -> Self {
        //Self es lo mismo que Bucketizer
        Bucketizer {
            buckets: Vec::new(),
        }
    }

    fn bucket(self, min: Option<f64>, max: Option<f64>, value: f64) -> Bucketizer{
        self
    }

    fn bucketize(&self, input: f64) -> Option<f64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Bucketizer;

    #[test]
    fn single_bucket_middle_values() {
        let bucketizer = Bucketizer::new().bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(bucketizer.bucketize(0.1), Some(0.5));
        assert_eq!(bucketizer.bucketize(999.999), None);
    }

    #[test]
    fn single_bucket_end_values() {
        let bucketizer = Bucketizer::new().bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(bucketizer.bucketize(0.0), Some(0.5));
        assert_eq!(bucketizer.bucketize(1.0), None);
    }

    #[test]
    fn multiple_buckets_closed_ends() {
        let b =
            Bucket::new()
                .bucket(Some(-1.0), Some(0, 0), -0.5)
                .bucket(Some(0.0), Some(1.0), 0.5);
        assert!(b.bucketize(0.0), Some(0.5));
        assert!(b.bucketize(-0.7), Some(-0.5));
        assert!(b.bucketize(9999.0), None);
    }

    #[test]
    fn multiple_buckets_opened_ends() {
        let b = Bucket::new()
            .bucket(Some(0.0), Some(1, 0), 0.5)
            .bucket(Some(1.0), None, 1.5);
        assert!(b.bucketize(0.0), Some(0.5));
        assert!(b.bucketize(-0.7), None);
        assert!(b.bucketize(9999.0), Some(1.5));
    }
}
