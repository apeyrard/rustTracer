mod vec3d;
use vec3d::Vec3d;

trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Vec3d>;
}

#[derive(Debug)]
struct Ray {
    origin: Vec3d,
    direction: Vec3d,
}

#[derive(Debug)]
struct Sphere {
    center: Vec3d,
    radius: f64,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Vec3d> {
        let L = &self.center - &ray.origin;

        Some(Vec3d::new(-8.0, 0.0, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ray_sphere_intersection() {
        let expected = Vec3d::new(-8.0, 0.0, 0.0);

        let ray = Ray {
            origin: Vec3d::new(-10.0, 0.0, 0.0),
            direction: Vec3d::new(1.0, 0.0, 0.0),
        };
        let sphere = Sphere {
            center: Vec3d::new(0.0, 0.0, 0.0),
            radius: 2.0,
        };

        let effective = sphere.intersect(&ray);

        assert_eq!(expected, effective.unwrap());
    }

    #[test]
    fn test_ray_sphere_intersection_not_at_origin() {
        let expected = Vec3d::new(25.5, 68.0, 50.0);

        let ray = Ray {
            origin: Vec3d::new(25.5, 108.0, 50.0),
            direction: Vec3d::new(0.0, -1.0, 0.0),
        };
        let sphere = Sphere {
            center: Vec3d::new(25.5, 28.0, 50.0),
            radius: 12.0,
        };

        let effective = sphere.intersect(&ray);

        assert_eq!(expected, effective.unwrap());
    }

    #[test]
    fn test_ray_sphere_intersection_non_unit_ray() {
        let expected = Vec3d::new(25.5, 68.0, 50.0);

        let ray = Ray {
            origin: Vec3d::new(25.5, 108.0, 50.0),
            direction: Vec3d::new(0.0, -10.0, 0.0),
        };
        let sphere = Sphere {
            center: Vec3d::new(25.5, 28.0, 50.0),
            radius: 12.0,
        };

        let effective = sphere.intersect(&ray);

        assert_eq!(expected, effective.unwrap());
    }
}
