#[cfg(test)]
mod tests{
    use crate::simulation::{grid::*, vector::{Float, Vector}};
    use approx::assert_relative_eq;

    #[test]
    fn test_flatten_index(){
        let grid = Grid::<f32, 2>::new(CoordInt([3, 4]), 1.0);
        assert_eq!(grid.flatten_index([0, 0]), 0);
        assert_eq!(grid.flatten_index([1, 0]), 4);
        assert_eq!(grid.flatten_index([0, 1]), 1);
        assert_eq!(grid.flatten_index([1, 1]), 5);
        assert_eq!(grid.flatten_index([2, 3]), 11);

        let grid = Grid::<f32, 3>::new(CoordInt([3, 4, 5]), 1.0);
        assert_eq!(grid.flatten_index([0, 0, 0]), 0);
        assert_eq!(grid.flatten_index([1, 0, 0]), 20);
        assert_eq!(grid.flatten_index([0, 1, 0]), 5);
        assert_eq!(grid.flatten_index([0, 0, 1]), 1);
        assert_eq!(grid.flatten_index([1, 2, 3]), 33);
    }

    #[test]
    fn test_vector_operations(){
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
    fn test_gradient(){
        let tolerance = 1e-10;

        let m = 5;
        let n = 8;

        let h0 = 0.3;
        let px = -0.1;
        let py = 0.4;

        let delta = 0.2;

        let mut grid = Grid::<Float, 2>::new(CoordInt([m, n]), delta);

        for i in 0..m{
            for j in 0..n{
                //grid.get_mut(&CoordInt([i, j])).map(|_| h0+px*(i as f64)*delta+py*(j as f64)*delta);
                *grid.get_mut(&CoordInt([i, j])).unwrap() = h0+px*(i as Float)*delta+py*(j as Float)*delta;
            }
        }

        for i in 1..m-1{
            for j in 1..n-1{
                let gradient = grid.gradient(CoordInt([i, j]));
                assert_relative_eq!(gradient, Vector([px, py]), epsilon=tolerance);
            }
        }


    }
}