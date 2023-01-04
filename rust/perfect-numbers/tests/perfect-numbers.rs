#![feature(test)]
extern crate test;

use perfect_numbers::*;

macro_rules! tests {
    ($property_test_func:ident {
        $( $(#[$attr:meta])* $test_name:ident( $( $param:expr ),* ); )+
    }) => {
        $(
            $(#[$attr])*
            #[test]
            fn $test_name() {
                $property_test_func($( $param ),* )
            }
        )+
    }
}

fn test_classification(num: u64, result: Classification) {
    assert_eq!(classify(num), Some(result));
}

#[test]
fn basic() {
    assert_eq!(classify(0), None);
}

tests! {
    test_classification {
        test_1(1, Classification::Deficient);
        test_2(2, Classification::Deficient);
        test_4(4, Classification::Deficient);
        test_6(6, Classification::Perfect);
        test_12(12, Classification::Abundant);
        test_28(28, Classification::Perfect);
        test_30(30, Classification::Abundant);
        test_32(32, Classification::Deficient);
        test_33550335(33_550_335, Classification::Abundant);
        test_33550336(33_550_336, Classification::Perfect);
        test_33550337(33_550_337, Classification::Deficient);
    }
}

#[bench]
fn bench_33550335_mine(b: &mut test::Bencher) {
    b.iter(|| classify(33_550_335))
}
// test bench_33550335_mine   ... bench:      39,075 ns/iter (+/- 3,131)
// without sqrt(num):
// test bench_33550335_mine   ... bench: 221,235,929 ns/iter (+/- 3,237,887)

#[bench]
fn bench_33550335_theirs(b: &mut test::Bencher) {
    b.iter(|| classify2(33_550_335))
}
// test bench_33550335_theirs ... bench: 191,248,728 ns/iter (+/- 1,583,579)
// using sqrt(num):
// test bench_33550335_theirs ... bench:      33,051 ns/iter (+/- 10,514)
