use super::util::pairwise;

#[test]
fn pairwise_empty() {
    let empty: Vec<i32> = Vec::new();
    itertools::assert_equal(pairwise(empty), vec![]);
}

#[test]
fn pairwise_single() {
    itertools::assert_equal(pairwise(vec![1]), vec![]);
}

#[test]
fn pairwise_two() {
    itertools::assert_equal(pairwise(vec![1, 2]), vec![(1, 2)]);
}

#[test]
fn pairwise_many() {
    itertools::assert_equal(
        pairwise(vec![1, 2, 3, 4, 5]),
        vec![(1, 2), (2, 3), (3, 4), (4, 5)],
    );
}
