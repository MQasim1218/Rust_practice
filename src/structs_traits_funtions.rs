use std::{f64::consts::PI, fmt};

struct Sphere {
    radius: i64,
    surface_area: f64,
    obj_volume: f64,
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Radius :: {}\t Area :: {}\nVolume :: {}",
            self.radius, self.surface_area, self.obj_volume
        )
    }
}

impl Sphere {
    fn _calc_volume(rad: f64) -> f64 {
        return (4 / 3) as f64 * (rad * rad * rad) * PI;
    }
    fn calc_area(rad: f64) -> f64 {
        return 4.0 * (rad * rad) * PI;
    }

    fn print_sphere(&self) {
        println!(
            "Sphere Radius is :: {}\nSphere Area :: {}\nSphere Volume :: {}\n",
            self.radius, self.surface_area, self.obj_volume
        );
    }

    fn new(rad: i64) -> Sphere {
        return Sphere {
            radius: rad,
            surface_area: Sphere::calc_area(rad as f64),
            obj_volume: Sphere::_calc_volume(rad as f64),
        };
    }
}

pub fn runner() {
    let s1 = Sphere {
        radius: 5,
        obj_volume: 0.0,
        surface_area: 0.0,
    };

    let s2 = Sphere::new(12);

    s1.print_sphere();
    s2.print_sphere();

    println!("{}", s2);
}
