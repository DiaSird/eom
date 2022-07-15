// Mass-Spring-Damper model
use crate::schemes::explicit::*;
use crate::traits::*;
use ndarray::{ArrayBase, DataMut, Ix1};

#[derive(Debug, Clone, Copy)]
pub struct MassSpringDamper {
    k: f64,       // spring
    c: f64,       // damper
    m: f64,       // mass
    x0: f64,      // initial position
    v0: f64,      // v0 = dx/dt|t=0
    n: i32,       // max loop
    delta_t: f64, // time steps
}

impl Default for MassSpringDamper {
    fn default() -> Self {
        MassSpringDamper {
            k: 1.0,        // [mN/mm]
            c: 1.0,        // [N/(mm/ms)]
            m: 1.0,        // [kg]
            x0: 0.0,       // [mm]
            v0: 1.0,       // [mm/ms]
            n: 1000,       //
            delta_t: 0.01, // [ms]
        }
    }
}

impl MassSpringDamper {
    pub fn new(k: f64, c: f64, m: f64, x0: f64, v0: f64, n: i32, delta_t: f64) -> Self {
        MassSpringDamper {
            k: k,
            c: c,
            m: m,
            x0: x0,
            v0: v0,
            n: n,
            delta_t: delta_t,
        }
    }
}

impl ModelSpec for MassSpringDamper {
    type Scalar = f64;
    type Dim = Ix1;

    fn model_size(&self) -> usize {
        2
    }
}

impl Explicit for MassSpringDamper {
    fn rhs<'a, S>(&mut self, v: &'a mut ArrayBase<S, Ix1>) -> &'a mut ArrayBase<S, Ix1>
    where
        S: DataMut<Elem = f64>,
    {
        let x1 = v[0];
        let x2 = v[1];

        // Define dx/dt = f(x)
        // v[0] = v;
        // v[1] = (-self.c * v - self.k * x) / self.m;
        v
    }
}
