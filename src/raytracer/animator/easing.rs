/// Tries to fit a curve where t is in the range [0, 1] and
/// a is t=0, b is t=0.33.., c is t=0.66.., and d is t=1.0
#[derive(Clone)]
pub struct Easing {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64
}

impl Easing {
    pub fn linear() -> Easing {
        Easing {
            a: 0.0,
            b: 1.0 / 3.0,
            c: 2.0 / 3.0,
            d: 1.0
        }
    }

    pub fn t(&self, t: f64) -> f64 {
        Easing::interpolate_cubic(self.a, self.b, self.c, self.d, t)
    }

    fn interpolate_cubic(a: f64, b: f64, c: f64, d: f64, t: f64) -> f64 {
        let abc = Easing::interpolate_quadratic(a, b, c, t);
        let bcd = Easing::interpolate_quadratic(b, c, d, t);

        Easing::interpolate_linear(abc, bcd, t)
    }

    fn interpolate_quadratic(a: f64, b: f64, c: f64, t: f64) -> f64 {
        let ab = Easing::interpolate_linear(a, b, t);
        let bc = Easing::interpolate_linear(b, c, t);

        Easing::interpolate_linear(ab, bc, t)
    }

    fn interpolate_linear(a: f64, b: f64, t: f64) -> f64 {
        (1.0 - t) * a + t * b
    }
}

#[test]
fn test_easing() {
    // This also tests Bezier easing as linear easing
    // is implemented using Bezier easing.
    let linear_easing = Easing::linear();;
    assert_eq!(linear_easing.t(0.0), 0.0);
    assert_eq!(linear_easing.t(0.25), 0.25);
    assert_eq!(linear_easing.t(0.5), 0.5);
    assert_eq!(linear_easing.t(0.75), 0.75);
    assert_eq!(linear_easing.t(1.0), 1.0);
}
