#[cfg(test)]
mod tests {
    use crate::simulation::{
        grid::{CoordInt, Grid},
        vector::{Float, Vector},
    };
    use approx::assert_relative_eq;

    #[test]
    fn test_flatten_index() {
        let grid = Grid::<f32, 2>::new(CoordInt([3, 4]), 1.0);
        assert_eq!(grid.flatten_index(&CoordInt([0, 0])), 0);
        assert_eq!(grid.flatten_index(&CoordInt([1, 0])), 4);
        assert_eq!(grid.flatten_index(&CoordInt([0, 1])), 1);
        assert_eq!(grid.flatten_index(&CoordInt([1, 1])), 5);
        assert_eq!(grid.flatten_index(&CoordInt([2, 3])), 11);

        let grid = Grid::<f32, 3>::new(CoordInt([3, 4, 5]), 1.0);
        assert_eq!(grid.flatten_index(&CoordInt([0, 0, 0])), 0);
        assert_eq!(grid.flatten_index(&CoordInt([1, 0, 0])), 20);
        assert_eq!(grid.flatten_index(&CoordInt([0, 1, 0])), 5);
        assert_eq!(grid.flatten_index(&CoordInt([0, 0, 1])), 1);
        assert_eq!(grid.flatten_index(&CoordInt([1, 2, 3])), 33);
    }

    #[test]
    fn test_vector_operations() {
        let v1 = Vector([1.0, 2.0]);
        let v2 = Vector([3.0, 4.0]);

        assert_eq!(v1 + v2, Vector([4.0, 6.0]));
        assert_eq!(v1 - v2, Vector([-2.0, -2.0]));
        assert_eq!(v1 * 2.0, Vector([2.0, 4.0]));
        assert_eq!(v1 / 2.0, Vector([0.5, 1.0]));
        assert_eq!(2.0 * v1, Vector([2.0, 4.0]));

        assert_eq!(v1, Vector([1.0, 2.0]));
        assert_eq!(v2, Vector([3.0, 4.0]));
    }

    #[test]
    fn test_gradient() {
        let tolerance = 1e-10;

        let m = 5;
        let n = 8;

        let h0 = 0.3;
        let px = -0.1;
        let py = 0.4;

        let delta = 0.2;

        let mut grid = Grid::new(CoordInt([m, n]), delta);

        for i in 0..m {
            for j in 0..n {
                //grid.get_mut(&CoordInt([i, j])).map(|_| h0+px*(i as f64)*delta+py*(j as f64)*delta);
                *grid.get_mut(&CoordInt([i, j])).unwrap() =
                    h0 + px * (i as Float) * delta + py * (j as Float) * delta;
            }
        }

        for i in 0..m {
            for j in 0..n {
                let expected_px = if i == 0 || i == m - 1 { px / 2.0 } else { px };
                let expected_py = if j == 0 || j == n - 1 { py / 2.0 } else { py };
                let gradient = grid.gradient(CoordInt([i, j]));
                assert_relative_eq!(
                    gradient,
                    Vector([expected_px, expected_py]),
                    epsilon = tolerance
                );
            }
        }
    }

    #[test]
    fn test_divergence() {
        let tolerance = 1e-10;

        let m = 5;
        let n = 8;

        let delta = 0.2;

        let kxx = 0.3;
        let kxy = -0.1;
        let kyx = 0.9;
        let kyy = -0.4;

        let mut grid = Grid::new(CoordInt([m, n]), delta);

        for i in 0..m {
            for j in 0..n {
                let x = i as Float * delta;
                let y = j as Float * delta;

                *grid.get_mut(&CoordInt([i, j])).unwrap() =
                    Vector([kxx * x + kxy * y, kyx * x + kyy * y]);
            }
        }

        for i in 0..m {
            for j in 0..n {
                let expected_kxx = if i == 0 || i == m - 1 { kxx / 2.0 } else { kxx };

                let expected_kyy = if j == 0 || j == n - 1 { kyy / 2.0 } else { kyy };

                let divergence = grid.divergence(CoordInt([i, j]));
                assert_relative_eq!(divergence, expected_kxx + expected_kyy, epsilon = tolerance);
            }
        }
    }
}
