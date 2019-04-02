use super::percentage::Percentage;
use super::point2d::Point2D;
use dim::si::Meter;
use dim::Dimensioned;

type Fdef = f64;
type Distance = Meter<Fdef>;

trait Lerp {
    fn lerp(a: Self, b: Self, t: Fdef) -> Self;
}

impl Lerp for Fdef {
    fn lerp(a: Self, b: Self, t: Fdef) -> Self {
        a * (1.0 - t) + b * t
    }
}

impl Lerp for Point2D {
    fn lerp(a: Self, b: Self, t: Fdef) -> Self {
        a + ((b - a) * t)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CurvePoint {
    percentage: Percentage,
    point: Point2D,
}

impl CurvePoint {
    fn new(percentage: Percentage, point: Point2D) -> Self {
        CurvePoint { percentage, point }
    }

    pub fn percentage(&self) -> Percentage {
        self.percentage
    }

    pub fn point(&self) -> Point2D {
        self.point
    }
}

#[derive(Clone, Debug)]
pub struct Curve {
    length: Distance,
    points: Vec<CurvePoint>,
}

impl Curve {
    pub fn new(points: Vec<Point2D>) -> Self {
        let length = get_total_length(&points);
        let mut points: Vec<_> = points
            .iter()
            .map(|p| CurvePoint::new(Percentage::lower(), *p))
            .collect();

        let mut acc = 0.0;
        for i in 1..points.len() {
            acc += points[i].point().distance(points[i - 1].point());
            points[i].percentage = Percentage::new_clamp(acc / length);
        }

        let length = Distance::new(length);

        Curve { length, points }
    }

    pub fn length(&self) -> Distance {
        self.length
    }

    pub fn distance_to_percentage(&self, distance: Distance) -> Percentage {
        let p = distance.value_unsafe() / self.length().value_unsafe();
        Percentage::new_clamp(p)
    }

    pub fn percentage_to_distance(&self, percentage: Percentage) -> Distance {
        percentage.value() * self.length()
    }

    pub fn get_location_at_distance(&self, distance: Distance) -> CurvePoint {
        let p = self.distance_to_percentage(distance);
        self.get_location_at_percentage(p)
    }

    pub fn get_location_at_percentage(&self, percentage: Percentage) -> CurvePoint {
        let num_points = self.points.len();
        let last_i = num_points - 1;

        let i = match self.get_point_index_for_input_value(percentage) {
            None => return *self.points.first().unwrap(),
            Some(i) => {
                if i == last_i {
                    if !is_looped(&self.points) {
                        return self.points[last_i];
                    } else if percentage >= self.points[last_i].percentage() {
                        return *self.points.first().unwrap();
                    }
                }
                i
            }
        };

        let is_loop_segment = is_looped(&self.points) && i == last_i;
        let next_i = if is_loop_segment { 0 } else { i + 1 };

        let prev = &self.points[i];
        let next = &self.points[next_i];

        let diff = if is_loop_segment {
            0.0
        } else {
            next.percentage().value() - prev.percentage().value()
        };

        if diff > 0.0 {
            let alpha = (percentage.value() - prev.percentage().value()) / diff;
            let point = Lerp::lerp(prev.point(), next.point(), alpha);
            CurvePoint::new(percentage, point)
        } else {
            self.points[i]
        }
    }

    fn get_point_index_for_input_value(&self, percentage: Percentage) -> Option<usize> {
        let num_points = self.points.len();
        let last_i = num_points - 1;

        if percentage < self.points.first().unwrap().percentage() {
            return None;
        }

        if percentage >= self.points[last_i].percentage() {
            return Some(last_i);
        }

        let mut min_i = 0;
        let mut max_i = num_points;

        while max_i - min_i > 1 {
            let mid = (min_i + max_i) / 2;

            if self.points[mid].percentage() <= percentage {
                min_i = mid;
            } else {
                max_i = mid;
            }
        }

        Some(min_i)
    }

    fn get_segment_length(&self, i: usize, param: Fdef) -> Fdef {
        let p0 = self.points[i].point();
        let p1 = if i == self.points.len() - 1 {
            self.points[0].point()
        } else {
            self.points[i + 1].point()
        };

        p1.distance(p0) * param
    }
}

fn is_looped<T: PartialEq>(points: &[T]) -> bool {
    if points.len() < 2 {
        return false;
    }
    points.first().unwrap() == points.last().unwrap()
}

fn get_total_length(points: &[Point2D]) -> Fdef {
    let mut acc = 0.0f64;
    for i in 1..points.len() {
        acc += points[i].distance(points[i - 1]);
    }
    acc
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn over_distance_gives_final_point() {
        let line = Curve::new(vec![Point2D::new(0.0, 0.0), Point2D::new(10.0, 15.0)]);

        assert_eq!(
            line.get_location_at_percentage(Percentage::new(1.0).unwrap()),
            CurvePoint::new(Percentage::new_clamp(1.0), Point2D::new(10.0, 15.0))
        );
    }

    #[test]
    fn mid_distance_gives_mid_points() {
        let line = Curve::new(vec![Point2D::new(0.0, 0.0), Point2D::new(3.0, 4.0)]);

        assert_eq!(
            line.get_location_at_percentage(Percentage::new(0.5).unwrap()),
            CurvePoint::new(Percentage::new_clamp(0.5), Point2D::new(1.5, 2.0))
        );
    }
}
