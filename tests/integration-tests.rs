extern crate bucketizer;
use bucketizer::Bucketizer;

#[test]
fn multiple_buckets_opened_ends() {
    let b = Bucketizer::new()
        .bucket(Some(0.0), Some(1.0), 0.5)
        .bucket(Some(1.0), None, 1.5);
    assert_eq!(b.bucketize(0.0), Some(0.5));
    assert_eq!(b.bucketize(-0.7), None);
    assert_eq!(b.bucketize(9999.0), Some(1.5));
}
