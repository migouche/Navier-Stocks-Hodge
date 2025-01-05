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

    #[test]
    fn test_laplace() {
        let tolerance = 1e-10;

        let a = 0.7;
        let b = -0.3;

        let delta = 0.2;

        let m = 5;
        let n = 8;

        let mut grid = Grid::new(CoordInt([m, n]), delta);

        for i in 0..m {
            for j in 0..n {
                let x = i as Float * delta;
                let y = j as Float * delta;

                *grid.get_mut(&CoordInt([i, j])).unwrap() = a * x * x * y + b * y * y;
            }
        }

        // TODO: test borders, we trust them for now

        for i in 1..m - 1 {
            for j in 1..n - 1 {
                let y = j as Float * delta;

                let laplace = grid.laplace(CoordInt([i, j]));
                let expected = 2.0 * a * y + 2.0 * b;
                assert_relative_eq!(laplace, expected, epsilon = tolerance);
            }
        }
    }

    #[test]
    fn test_get_at() {
        let epsilon = 1e-10;

        let m = 3;
        let n = 3;

        let a = 0.7;
        let b = -0.3;
        let f = |x: Float, y: Float| a * x + b * y;

        let delta = 5.0;

        let mut grid = Grid::new(CoordInt([m, n]), delta);

        for i in 0..m {
            for j in 0..n {
                let x = delta * i as Float;
                let y = delta * j as Float;

                *grid.get_mut(&CoordInt([i, j])).unwrap() = f(x, y);
            }
        }

        let test_set = [(0.0, 0.0), (1.3, 3.7), (8.1, 1.6), (5.0, 5.0), (7.9, 6.3)];

        for (x, y) in test_set.iter() {
            let expected = f(*x, *y);
            let actual = grid.get_at(&Vector([*x, *y]));
            assert_relative_eq!(actual, expected, epsilon = epsilon);
        }

        // test out of bounds
        let test_set = [
            ((0.0, -0.1), (0.0, 0.0)),
            ((-0.1, 0.0), (0.0, 0.0)),
            ((0.0, 19.0), (0.0, 10.0)),
            ((19.0, 0.0), (10.0, 0.0)),
            ((19.0, 19.0), (10.0, 10.0)),
        ];

        for ((x, y), (expected_x, expected_y)) in test_set.iter() {
            let expected = f(*expected_x, *expected_y);
            let actual = grid.get_at(&Vector([*x, *y]));
            assert_relative_eq!(actual, expected, epsilon = epsilon);
        }
    }

    #[test]
    fn test_from_coord_int() {
        let delta = 0.1;
        let coord = CoordInt([1, 2]);
        let v = Vector::from_coord_int(coord, delta);
        assert_eq!(v, Vector([0.1, 0.2]));
    }

    #[test]
    fn test_advect() {
        let tolerance = 1e-10;

        let m = 3;
        let n = 3;

        let a = 0.7;
        let b = -0.3;

        let delta = 0.1;
        let dt = 0.05;

        let v = [0.5, 1.0];

        let mut grid = Grid::new(CoordInt([m, n]), delta);
        let mut v_grid = Grid::new(CoordInt([m, n]), delta);

        for i in 0..m {
            for j in 0..n {
                let x = i as Float * delta;
                let y = j as Float * delta;

                *grid.get_mut(&CoordInt([i, j])).unwrap() = a * x + b * y;
                *v_grid.get_mut(&CoordInt([i, j])).unwrap() = Vector([10000000.0, 10000000.0]);
            }
        }

        *v_grid.get_mut(&CoordInt([1, 1])).unwrap() = Vector(v);

        let x = delta - dt * v[0];
        let y = delta - dt * v[1];

        assert_relative_eq!(
            grid.advect(&v_grid, CoordInt([1, 1]), dt),
            a * x + b * y,
            epsilon = tolerance
        );
    }

    #[test]
    fn test_iter(){
        // 1D grid
        let m = 5;
        let mut grid = Grid::new(CoordInt([m]), 1.0);
        let mut count = 0;
        for i in 0..m {
            *grid.get_mut(&CoordInt([i])).unwrap() = count;
            count += 1;
        }

        let mut new_count = 0;

        for (coord, value) in grid.into_iter(){
            assert_eq!(*value, new_count);
            new_count += 1;
            println!("coord: {:?}, value: {}\n", coord, value);
        }

        assert_eq!(count, new_count);

        
        // 2D grid
        let m = 5;
        let n = 8;
        
        let mut grid = Grid::new(CoordInt([m, n]), 1.0);
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                *grid.get_mut(&CoordInt([i, j])).unwrap() = count;
                count += 1;
            }
        }

        let mut new_count = 0;

        for (coord, value) in grid.into_iter(){
            assert_eq!(*value, new_count);
            new_count += 1;
            println!("coord: {:?}, value: {}\n", coord, value);
        }

        assert_eq!(count, new_count);


        // 3D grid

        let m = 5;
        let n = 8;
        let o = 3;

        let mut grid = Grid::new(CoordInt([m, n, o]), 1.0);
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                for k in 0..o {
                    *grid.get_mut(&CoordInt([i, j, k])).unwrap() = count;
                    count += 1;
                }
            }
        }

        let mut new_count = 0;

        for (coord, value) in grid.into_iter(){
            assert_eq!(*value, new_count);
            new_count += 1;
            println!("coord: {:?}, value: {}\n", coord, value);
        }

        assert_eq!(count, new_count);
    }
}
