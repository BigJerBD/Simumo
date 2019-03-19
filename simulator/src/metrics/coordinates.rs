use std::ops::Sub;
use graphics::radians::Radians;



const EARTH_RADIUS: f64 =  6371.0088;
const PI: f64 = std::f64::consts::PI;

///used for modifiability
type Fdef = f64;
type Lon = Fdef;
type Lat = Fdef;


/// represent a coordinate
/// using the earth's longitude and latitude
///
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PolarCoord(pub Lat, pub Lon);
impl PolarCoord {
    pub fn from_cartesian(coord:CartesianCoord) -> Self {
        Self (
            coord.1 / EARTH_RADIUS / ( PI / 180.),
            coord.0 / EARTH_RADIUS / ( PI / 180.) / (2./ 3.)
        )
    }

    pub fn from_point(Point2D{x,y} : Point2D) {
        Self (y,x)
    }

}
impl Sub for PolarCoord {
    type Output = PolarCoord;

    fn sub(self, other: PolarCoord) -> Self::Output {
        PolarCoord(self.0 - other.0, self.1 - other.1)
    }
}

/// represent a coordinate
/// using a flat X and Y surface
///
pub struct  CartesianCoord(pub Fdef, pub Fdef);
impl CartesianCoord{
    pub fn from_polar(coord:PolarCoord) -> Self {
        Self (
            coord.1 * EARTH_RADIUS * ( PI / 180.) * (2./3.),
            coord.0 * EARTH_RADIUS * ( PI / 180.)
        )
    }

    pub fn from_point(Point2D{x,y} : Point2D) {
        Self (y,x)
    }
}
impl Sub for CartesianCoord {
    type Output = CartesianCoord;
    fn sub(self, other: CartesianCoord) -> Self::Output {
        CartesianCoord(self.0 - other.0, self.1 - other.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn polar_to_cartesian(){
        let pcoord =  PolarCoord(45.0,90.0);
        let ccoord =  CartesianCoord::from_polar(pcoord);

        assert_eq!(5003.778610508981,ccoord.1);
        assert_eq!(6671.704814011974,ccoord.0);
    }

    #[test]
    fn cartesian_to_polar(){
        let ccoord =  CartesianCoord(6671.704814011974,5003.778610508981);
        let pcoord =  PolarCoord::from_cartesian(ccoord);
        assert_eq!(45.,pcoord.0.ceil());
        assert_eq!(90.,pcoord.1.ceil());
    }

    #[test]
    fn polar_to_cartesian_30_90(){
        let pcoord =  PolarCoord(30.0,90.0);
        let ccoord =  CartesianCoord::from_polar(pcoord);

        assert_eq!(3335.852407005987,ccoord.1);
        assert_eq!(6671.704814011974,ccoord.0);



    }


}