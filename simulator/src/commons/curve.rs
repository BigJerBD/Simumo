use super::percentage::Percentage;
use super::point2d::Point2D;
use std::ops::Sub;

type Fdef = f64;

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

#[derive(Clone, Debug, PartialEq)]
pub struct CurvePoint<T> {
    in_val: Fdef,
    out_val: T,
}
impl<T: Sub> CurvePoint<T> {
    fn new(in_val: Fdef, out_val: T) -> Self {
        CurvePoint { in_val, out_val }
    }
}

#[derive(Clone, Debug)]
pub struct Curve {
    points: Vec<CurvePoint<Point2D>>,
}

impl Curve {
    pub fn new(points: Vec<Point2D>) -> Self {
        let mut points: Vec<_> = points.iter().map(|p| CurvePoint::new(0.0, *p)).collect();

        let length = get_total_length(&points);
        let mut acc = 0.0;
        for i in 1..points.len() {
            acc += points[i].out_val.distance(points[i - 1].out_val);
            points[i].in_val = acc / length;
        }

        Curve { points }
    }

    pub fn get_location_at_distance_along_curve(&self, distance: Percentage) -> Point2D {
        eval(&self.points, distance.value())
    }
}

fn eval<T>(points: &[CurvePoint<T>], in_val: Fdef) -> T
where
    T: Sub + Clone + Lerp + PartialEq,
{
    let num_points = points.len();
    let last_i = num_points - 1;

    let i = match get_point_index_for_input_value(points, in_val) {
        None => return points.first().unwrap().out_val.clone(),
        Some(i) => {
            if i == last_i {
                if !is_looped(points) {
                    return points[last_i].out_val.clone();
                } else if in_val >= points[last_i].in_val {
                    return points.first().unwrap().out_val.clone();
                }
            }
            i
        }
    };

    let is_loop_segment = is_looped(points) && i == last_i;
    let next_i = if is_loop_segment { 0 } else { i + 1 };

    let prev = &points[i];
    let next = &points[next_i];

    let diff = if is_loop_segment {
        0.0
    } else {
        next.in_val - prev.in_val
    };

    if diff > 0.0 {
        let alpha = (in_val - prev.in_val) / diff;
        Lerp::lerp(prev.out_val.clone(), next.out_val.clone(), alpha)
    } else {
        points[i].out_val.clone()
    }
}

fn is_looped<T: PartialEq>(points: &[T]) -> bool {
    if points.len() < 2 {
        return false;
    }
    points.first().unwrap() == points.last().unwrap()
}

fn get_point_index_for_input_value<T: Sub>(
    points: &[CurvePoint<T>],
    in_val: Fdef,
) -> Option<usize> {
    let num_points = points.len();
    let last_i = num_points - 1;

    if in_val < points.first().unwrap().in_val {
        return None;
    }

    if in_val >= points[last_i].in_val {
        return Some(last_i);
    }

    let mut min_i = 0;
    let mut max_i = num_points;

    while max_i - min_i > 1 {
        let mid = (min_i + max_i) / 2;

        if points[mid].in_val <= in_val {
            min_i = mid;
        } else {
            max_i = mid;
        }
    }

    Some(min_i)
}

fn get_total_length(points: &[CurvePoint<Point2D>]) -> Fdef {
    let mut acc = 0.0f64;
    for i in 1..points.len() {
        acc += points[i].out_val.distance(points[i - 1].out_val);
    }
    acc
}

fn get_segment_length(points: &[CurvePoint<Point2D>], i: usize, param: Fdef) -> Fdef {
    let p0 = points[i].out_val;
    let p1 = if i == points.len() - 1 {
        points[0].out_val
    } else {
        points[i + 1].out_val
    };

    p1.distance(p0) * param
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn over_distance_gives_final_point() {
        let line = Curve::new(vec![Point2D::new(0.0, 0.0), Point2D::new(10.0, 15.0)]);

        assert_eq!(
            line.get_location_at_distance_along_curve(Percentage::new(1.0).unwrap()),
            Point2D::new(10.0, 15.0)
        );
    }

    #[test]
    fn mid_distance_gives_mid_points() {
        let line = Curve::new(vec![Point2D::new(0.0, 0.0), Point2D::new(3.0, 4.0)]);

        assert_eq!(
            line.get_location_at_distance_along_curve(Percentage::new(0.5).unwrap()),
            Point2D::new(1.5, 2.0)
        );
    }
}