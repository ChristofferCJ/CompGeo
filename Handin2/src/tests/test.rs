#[cfg(test)]
mod tests {
    use crate::app::structs::*;
    use crate::{gift_wrapping, grahams_scan};
    use crate::app::data_generator::{generate_points_circle, generate_points_curve, generate_points_square};
    use crate::tests::test_data::*;


    #[test]
    fn test_line_check_side() {
        let line = Line {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 1.0, y: 0.0 },
        };
        let line_reversed = Line {
            start: Point { x: 1.0, y: 0.0 },
            end: Point { x: 0.0, y: 0.0 },
        };
        let point_top = Point { x: 0.5, y: 0.5 };
        let point_bottom = Point { x: -1.0, y: -0.1 };
        let point_intersect = Point { x: 2.0, y: 0.0 };

        assert_eq!(line.check_side(&point_top), Side::TOP);
        assert_eq!(line.check_side(&point_bottom), Side::BOTTOM);
        assert_eq!(line.check_side(&point_intersect), Side::INTERSECT);

        assert_eq!(line_reversed.check_side(&point_top), Side::BOTTOM);
        assert_eq!(line_reversed.check_side(&point_bottom), Side::TOP);
        assert_eq!(line_reversed.check_side(&point_intersect), Side::INTERSECT);
    }

    #[test]
    fn test_gift_wrapping() {
        let result1 = gift_wrapping(&points_1());
        assert_eq!(result1, points_1_expected());

        let result2 = gift_wrapping(&points_2());
        assert_eq!(result2, points_2_expected());
    }

    #[test]
    fn test_graham_scan() {
        let result = grahams_scan(&points_1());
        assert_eq!(result, Some(points_1_expected()));

        let result2 = grahams_scan(&points_2());
        assert_eq!(result2, Some(points_2_expected()));
    }

    #[test]
    fn test_compare_algorithms_with_random_square() {

        for i in 0..100 {
            let data = generate_points_square(1000, i);

            let result_gift = gift_wrapping(&data);
            let result_graham = grahams_scan(&data);

            assert_eq!(result_graham, Some(result_gift));
        }
    }

    #[test]
    fn test_compare_algorithms_with_random_circle() {

        for i in 0..100 {
            let data = generate_points_circle(1000, i);

            let result_gift = gift_wrapping(&data);
            let result_graham = grahams_scan(&data);

            assert_eq!(result_graham, Some(result_gift));
        }
    }

    #[test]
    fn test_compare_algorithms_with_random_curve() {

        for i in 0..100 {
            let data = generate_points_curve(1000, i);

            let result_gift = gift_wrapping(&data);
            let result_graham = grahams_scan(&data);

            assert_eq!(result_graham, Some(result_gift));
        }
    }
}