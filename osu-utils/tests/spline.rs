use osu_types::CurveType;
use osu_utils::Spline;

#[test]
fn test_spline_points() {
    let spline_points_list = include_str!("spline_points_list.in");
    let mut expected_spline_points = Vec::new();
    for line in spline_points_list.lines() {
        let mut parts = line.split(",");
        let x = parts.next().unwrap().parse::<f32>().unwrap();
        let y = parts.next().unwrap().parse::<f32>().unwrap();
        expected_spline_points.push((x, y));
    }

    for _ in 0..10000 {
        let spline = Spline::from_control(
            CurveType::Bezier,
            &[
                mint::Point2 { x: 0, y: 0 },
                mint::Point2 { x: 0, y: 10 },
                mint::Point2 { x: 20, y: 5 },
            ],
            Some(100.0),
        );

        let expected_points = expected_spline_points.len();
        let actual_points = spline.spline_points.len();
        assert_eq!(
            expected_points, actual_points,
            "expected {} points, got {}",
            expected_points, actual_points
        );

        for (i, (mint::Point2 { x: ax, y: ay }, (ex, ey))) in spline
            .spline_points
            .iter()
            .zip(expected_spline_points.iter())
            .enumerate()
        {
            assert!(
                (ex - ax).abs() < 0.001,
                "ln{}x: expected {}, got {}",
                i,
                ex,
                ax
            );
            assert!(
                (ey - ay).abs() < 0.001,
                "ln{}y: expected {}, got {}",
                i,
                ex,
                ax
            );
        }
    }
}
