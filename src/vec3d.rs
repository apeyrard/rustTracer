use std::ops::Sub;

#[derive(Debug, PartialEq)]
pub struct Vec3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d { x: x, y: y, z: z }
    }

    pub fn almost_eq(&self, other: &Vec3d) -> bool {
        let epsilon = 0.000000000000001;
        self.x - other.x < epsilon && self.y - other.y < epsilon && self.z - other.z < epsilon
    }
}

impl Sub for &Vec3d {
    type Output = Vec3d;

    fn sub(self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        let a = Vec3d::new(1.9, -3.9, 32.7);
        let b = Vec3d::new(1.9, -3.9, 32.7);
        assert_eq!(a, b);
    }

    #[test]
    fn substraction() {
        let expected = Vec3d::new(0.0, 0.0, 0.0);

        let a = Vec3d::new(1.9, -3.9, 32.7);
        let b = Vec3d::new(1.9, -3.9, 32.7);

        let effective = &a - &b;

        assert_eq!(expected.almost_eq(&effective), true);
    }

    #[test]
    fn substraction_other_case() {
        let expected = Vec3d::new(-0.9, -2.0, 24.1);

        let a = Vec3d::new(1.0, -3.9, 32.7);
        let b = Vec3d::new(1.9, -1.9, 8.6);

        let effective = &a - &b;

        assert_eq!(expected.almost_eq(&effective), true);
    }

    #[test]
    fn when_equal_then_almost_equal_is_true() {
        let a = Vec3d::new(1.0, 5.6, -98.5478);
        let b = Vec3d::new(1.0, 5.6, -98.5478);

        assert_eq!(a.almost_eq(&b), true);
    }

    #[test]
    fn when_not_equal_then_almost_equal_is_false() {
        let a = Vec3d::new(1.0, 5.6, -98.5478);
        let b = Vec3d::new(8.0, 4.5, -8.5478);

        assert_eq!(a.almost_eq(&b), false);
    }

    #[test]
    fn when_almost_equal_then_almost_equal_is_true() {
        let a = Vec3d::new(-0.8999999999999999, 5.6, -98.5478);
        let b = Vec3d::new(-0.9, 5.6, -98.5478);

        assert_eq!(a.almost_eq(&b), true);
    }

    #[test]
    fn epsilon_is_actually_small() {
        let a = Vec3d::new(-0.89999999999999, 5.6, -98.5478);
        let b = Vec3d::new(-0.9, 5.6, -98.5478);

        assert_eq!(a.almost_eq(&b), false);
    }
}
