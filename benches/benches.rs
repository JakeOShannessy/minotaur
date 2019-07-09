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
                let mut grid = minotaur::Grid::new(10, 10);
                grid.sidewinder(None)
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                let mut grid = minotaur::Grid::new(100, 100);
                grid.sidewinder(None)
            });
        }
    }

    mod binary_tree {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                let mut grid = minotaur::Grid::new(10, 10);
                grid.binary_tree(None)
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                let mut grid = minotaur::Grid::new(100, 100);
                grid.binary_tree(None)
            });
        }
    }

    mod aldous_broder {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                let mut grid = minotaur::Grid::new(10, 10);
                grid.aldous_broder(None)
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                let mut grid = minotaur::Grid::new(100, 100);
                grid.aldous_broder(None)
            });
        }
    }

    mod wilsons {
        use super::*;

        #[bench]
        fn generate_10_x_10(b: &mut Bencher) {
            b.iter(|| {
                let mut grid = minotaur::Grid::new(10, 10);
                grid.wilsons(None)
            });
        }

        #[bench]
        fn generate_100_x_100(b: &mut Bencher) {
            b.iter(|| {
                let mut grid = minotaur::Grid::new(100, 100);
                grid.wilsons(None)
            });
        }
    }
}
