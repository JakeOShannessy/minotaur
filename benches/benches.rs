#![feature(test)]

extern crate minotaur;

mod bench {
    extern crate test;
    use self::test::Bencher;

    mod sidewinder {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                // Note lack of `;` (could also use an explicit `return`).
                minotaur::Grid::sidewinder(10, 10, None)
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                // Note lack of `;` (could also use an explicit `return`).
                minotaur::Grid::sidewinder(100, 100, None)
            });
        }
    }

    mod binary_tree {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                // Note lack of `;` (could also use an explicit `return`).
                minotaur::Grid::binary_tree(10, 10, None)
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                // Note lack of `;` (could also use an explicit `return`).
                minotaur::Grid::binary_tree(100, 100, None)
            });
        }

        mod aldous_broder {
            use super::*;

            #[bench]
            fn generate_10_x_10(b: &mut Bencher) {
                b.iter(|| {
                    // Note lack of `;` (could also use an explicit `return`).
                    minotaur::Grid::aldous_broder(10, 10, None)
                });
            }

            #[bench]
            fn generate_100_x_100(b: &mut Bencher) {
                b.iter(|| {
                    // Note lack of `;` (could also use an explicit `return`).
                    minotaur::Grid::aldous_broder(100, 100, None)
                });
            }
        }
    }
}
