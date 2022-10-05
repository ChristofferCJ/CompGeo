use crate::app::structs::*;

pub fn points_1() -> Vec<Point> {
    return vec![
        Point { x: 0.01, y: 0.0 },
        Point { x: 0.02, y: 1.0 },
        Point { x: 1.03, y: 1.0 },
        Point { x: 1.04, y: 0.0 },
        Point { x: 0.55, y: 0.5 },
        Point { x: -0.5, y: 0.5 },
        Point { x: 0.56, y: 0.5 },
        Point { x: 0.57, y: 0.5 },
    ];
}

pub fn points_1_expected() -> Vec<Point> {
    return vec![
        Point { x: -0.5, y: 0.5 },
        Point { x: 0.02, y: 1.0 },
        Point { x: 1.03, y: 1.0 },
        Point { x: 1.04, y: 0.0 },
        Point { x: 0.01, y: 0.0 },
    ];
}

pub fn points_2() -> Vec<Point> {
    return vec![
        Point { x: 1.0, y: 1.0 },
        Point { x: 1.5, y: 1.6 },
        Point { x: 2.0, y: 2.0 },
        Point { x: 2.5, y: 2.9 },
        Point { x: 3.0, y: 4.0 },
        Point { x: 4.0, y: 5.0 },

        Point { x: 5.0, y: 5.5 },

        Point { x: 6.0, y: 5.0 },
        Point { x: 7.0, y: 4.0 },
        Point { x: 7.5, y: 2.9 },
        Point { x: 8.0, y: 2.0 },
        Point { x: 8.5, y: 1.6 },
        Point { x: 9.0, y: 1.0 },

        Point { x: 9.0, y: -1.0 },
        Point { x: 8.5, y: -1.6 },
        Point { x: 8.0, y: -2.0 },
        Point { x: 7.5, y: -2.9 },
        Point { x: 7.0, y: -4.0 },
        Point { x: 6.0, y: -5.0 },

        Point { x: 5.0, y: -5.5 },

        Point { x: 4.0, y: -5.0 },
        Point { x: 3.0, y: -4.0 },
        Point { x: 2.5, y: -2.9 },
        Point { x: 2.0, y: -2.0 },
        Point { x: 1.5, y: -1.6 },
        Point { x: 1.0, y: -1.0 },
    ];
}

pub fn points_2_expected() -> Vec<Point> {
    return vec![
        Point { x: 1.0, y: 1.0,},
        Point { x: 3.0, y: 4.0,},
        Point { x: 4.0, y: 5.0,},
        Point { x: 5.0, y: 5.5 },
        Point { x: 6.0, y: 5.0,},
        Point { x: 7.0, y: 4.0,},
        Point { x: 9.0, y: 1.0,},
        Point { x: 9.0, y: -1.0,},
        Point { x: 7.0, y: -4.0,},
        Point { x: 6.0, y: -5.0,},
        Point { x: 5.0, y: -5.5 },
        Point { x: 4.0, y: -5.0,},
        Point { x: 3.0, y: -4.0,},
        Point { x: 1.0, y: -1.0,},
    ];
}