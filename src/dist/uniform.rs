use super::Distribution;

/// A continuous uniform distribution on an interval [`a`, `b`].
pub struct Uniform {
    /// The left endpoint of the support.
    pub a: f64,
    /// The right endpoint of the support.
    pub b: f64,
}

impl Uniform {
    #[inline]
    /// Creates a uniform distribution on the interval [`a`, `b`].
    pub fn new(a: f64, b: f64) -> Uniform {
        Uniform { a: a, b: b }
    }
}

impl Distribution<f64> for Uniform {
    fn cdf(&self, x: f64) -> f64 {
        if x <= self.a {
            0.0
        } else if self.b <= x {
            1.0
        } else {
            (x - self.a) / (self.b - self.a)
        }
    }

    #[inline]
    fn inv_cdf(&self, p: f64) -> f64 {
        self.a + (self.b - self.a) * p
    }

    #[inline]
    fn sample(&self) -> f64 {
        self.a + (self.b - self.a) * ::std::rand::random()
    }
}

#[cfg(test)]
mod test {
    use super::super::{Distribution, Sampler};
    use super::Uniform;

    #[test]
    fn cdf() {
        let u = Uniform::new(-1.0, 1.0);
        let x = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
        let p = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        assert_eq!(x.iter().map(|&x| u.cdf(x)).collect::<Vec<_>>(), p);
    }

    #[test]
    fn inv_cdf() {
        let u = Uniform::new(-1.0, 1.0);
        let x = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
        let p = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        assert_eq!(p.iter().map(|&p| u.inv_cdf(p)).collect::<Vec<_>>(), x);
    }

    #[test]
    fn sample() {
        for u in Sampler(&Uniform::new(7.0, 42.0)).take(100) {
            assert!(7.0 <= u && u <= 42.0);
        }
    }
}
