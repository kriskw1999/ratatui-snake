use crate::coord::Coord;

fn is_point_on_line(point_coord: &Coord, (start_coord, end_coord): (&Coord, &Coord)) -> bool {
    let Coord { x: px, y: py } = point_coord;
    let Coord { x: x1, y: y1 } = start_coord;
    let Coord { x: x2, y: y2 } = end_coord;

    let cross_product = (py - y1) * (x2 - x1) - (px - x1) * (y2 - y1);
    if cross_product.abs() > f64::EPSILON {
        return false;
    }

    let dot_product = (px - x1) * (x2 - x1) + (py - y1) * (y2 - y1);
    if dot_product < 0.0 {
        return false;
    }

    let squared_length = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
    if dot_product > squared_length {
        return false;
    }

    true
}

pub fn check_collisions(collidable1: &dyn Collidable, collidable2: &dyn Collidable) -> bool {
    let border1 = collidable1.get_border();
    let border2 = collidable2.get_border();

    // collision between points
    if border1.len() == 1 && border2.len() == 1 {
        if border1[0].x == border2[0].x && border1[0].y == border2[0].y {
            return true;
        }
    }
    // collision between point and line
    else if border1.len() == 1 || border2.len() == 1 {
        let (point, borders) = if border1.len() == 1 {
            (&border1[0], &border2)
        } else {
            (&border2[0], &border1)
        };

        for line in borders.windows(2) {
            if is_point_on_line(point, (&line[0], &line[1])) {
                return true;
            }
        }
    }
    // collision between two lines or shapes
    else {
        for line1 in border1.windows(2) {
            for line2 in border2.windows(2) {
                let x1 = line1[0].x;
                let y1 = line1[0].y;
                let x2 = line1[1].x;
                let y2 = line1[1].y;

                let x3 = line2[0].x;
                let y3 = line2[0].y;
                let x4 = line2[1].x;
                let y4 = line2[1].y;

                let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

                if den == 0.0 {
                    continue;
                }

                let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
                let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

                if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
                    return true;
                }
            }
        }
    }

    false
}

pub trait Collidable {
    fn get_border(&self) -> Vec<Coord>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCollidable {
        border: Vec<Coord>,
    }

    impl Collidable for TestCollidable {
        fn get_border(&self) -> Vec<Coord> {
            self.border.clone()
        }
    }

    #[test]
    fn test_collision_manager() {
        let collidable1 = TestCollidable {
            border: vec![
                Coord::new(0.0, 0.0),
                Coord::new(0.0, 5.0),
                Coord::new(5.0, 5.0),
                Coord::new(5.0, 0.0),
            ],
        };

        let collidable2 = TestCollidable {
            border: vec![
                Coord::new(0.0, 5.0),
                Coord::new(1.5, 0.0),
                Coord::new(2.5, 2.5),
                Coord::new(3.5, 0.0),
                Coord::new(5.0, 5.0),
            ],
        };

        let collided = check_collisions(&collidable1, &collidable2);

        assert_eq!(collided, true);
    }

    #[test]
    fn test_no_collision() {
        let collidable1 = TestCollidable {
            border: vec![
                Coord::new(0.0, 0.0),
                Coord::new(0.0, 1.0),
                Coord::new(1.0, 1.0),
                Coord::new(1.0, 0.0),
            ],
        };

        let collidable2 = TestCollidable {
            border: vec![
                Coord::new(2.0, 0.0),
                Coord::new(3.0, 0.0),
                Coord::new(2.0, 1.0),
                Coord::new(3.0, 1.0),
            ],
        };

        let collided = check_collisions(&collidable1, &collidable2);

        assert_eq!(collided, false);
    }

    #[test]
    fn test_no_collision_with_same_border() {
        let collidable1 = TestCollidable {
            border: vec![
                Coord::new(0.0, 0.0),
                Coord::new(0.0, 1.0),
                Coord::new(1.0, 1.0),
                Coord::new(1.0, 0.0),
            ],
        };

        let collidable2 = TestCollidable {
            border: vec![Coord::new(0.0, 0.0)],
        };

        let collided = check_collisions(&collidable1, &collidable2);

        assert_eq!(collided, true);
    }

    #[test]
    fn test_collision_of_two_points() {
        let collidable1 = TestCollidable {
            border: vec![Coord::new(0.0, 0.0)],
        };

        let collidable2 = TestCollidable {
            border: vec![Coord::new(0.0, 0.0)],
        };

        let collided = check_collisions(&collidable1, &collidable2);

        assert_eq!(collided, true);
    }

    #[test]
    fn test_collision_of_points_over_a_line() {
        let collidable1 = TestCollidable {
            border: vec![Coord::new(0.0, 0.0), Coord::new(9.0, 0.0)],
        };

        let collidable2 = TestCollidable {
            border: vec![Coord::new(5.0, 0.0)],
        };

        let collided = check_collisions(&collidable1, &collidable2);

        assert_eq!(collided, true);
    }
}
