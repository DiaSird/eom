// explicit schemes
// Ref. https://github.com/termoshtt/eom
use ndarray::*;
use ndarray_linalg::*;
use crate::traits::*;

#[derive(Debug, Clone)]
pub struct RK4<F: Explicit> {
    f: F,
    dt: <F::Scalar as Scalar>::Real,
    x: Array<F::Scalar, F::Dim>,
    k1: Array<F::Scalar, F::Dim>,
    k2: Array<F::Scalar, F::Dim>,
    k3: Array<F::Scalar, F::Dim>,
}

impl<A: Scalar, F: Explicit<Scalar = A>> TimeStep for RK4<F> {
    type Time = A::Real;

    fn get_dt(&self) -> Self::Time {
        self.dt
    }
    fn set_dt(&mut self, dt: Self::Time) {
        self.dt = dt;
    }
}

impl<F: Explicit> Scheme for RK4<F> {
    type Core = F;

    fn new(f: F, dt: Self::Time) -> Self {
        let x = Array::zeros(f.model_size());
        let k1 = Array::zeros(f.model_size());
        let k2 = Array::zeros(f.model_size());
        let k3 = Array::zeros(f.model_size());
        Self {
            f,
            dt,
            x,
            k1,
            k2,
            k3,
        }
    }
    fn core(&self) -> &Self::Core {
        &self.f
    }
    fn core_mut(&mut self) -> &mut Self::Core {
        &mut self.f
    }
}

impl<F: Explicit> ModelSpec for RK4<F> {
    type Scalar = F::Scalar;
    type Dim = F::Dim;

    fn model_size(&self) -> <Self::Dim as Dimension>::Pattern {
        self.f.model_size()
    }
}

impl<F: Explicit> TimeEvolution for RK4<F> {
    fn iterate<'a, S>(&mut self, x: &'a mut ArrayBase<S, F::Dim>) -> &'a mut ArrayBase<S, Self::Dim>
    where
        S: DataMut<Elem = Self::Scalar>,
    {
        let two = F::Scalar::real(2.0);
        let dt = self.dt;
        let dt_2 = self.dt * F::Scalar::real(0.5);
        let dt_6 = self.dt / F::Scalar::real(6.0);
        self.x.zip_mut_with(x, |buf, x| *buf = *x);
        // k1
        let k1 = self.f.rhs(x);
        self.k1.zip_mut_with(k1, |buf, k1| *buf = *k1);
        Zip::from(&mut *k1).and(&self.x).for_each(|k1, &x| {
            *k1 = k1.mul_real(dt_2) + x;
        });
        // k2
        let k2 = self.f.rhs(k1);
        self.k2.zip_mut_with(k2, |buf, k| *buf = *k);
        Zip::from(&mut *k2).and(&self.x).for_each(|k2, &x| {
            *k2 = x + k2.mul_real(dt_2);
        });
        // k3
        let k3 = self.f.rhs(k2);
        self.k3.zip_mut_with(k3, |buf, k| *buf = *k);
        Zip::from(&mut *k3).and(&self.x).for_each(|k3, &x| {
            *k3 = x + k3.mul_real(dt);
        });
        let k4 = self.f.rhs(k3);
        Zip::from(&mut *k4)
            .and(&self.x)
            .and(&self.k1)
            .and(&self.k2)
            .and(&self.k3)
            .for_each(|k4, &x, &k1, &k2, &k3| {
                *k4 = x + (k1 + (k2 + k3).mul_real(two) + *k4).mul_real(dt_6);
            });
        k4
    }
}