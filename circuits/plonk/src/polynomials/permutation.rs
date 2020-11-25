/*****************************************************************************************************************

This source file implements permutation constraint polynomial.

*****************************************************************************************************************/

use algebra::{FftField, SquareRootField};
use ff_fft::{Evaluations, DensePolynomial, Radix2EvaluationDomain as D, DenseOrSparsePolynomial};
use oracle::{utils::{EvalUtils, PolyUtils}, rndoracle::ProofError};
use crate::scalars::{ProofEvaluations, RandomOracles};
use crate::polynomial::WitnessOverDomains;
use crate::constraints::ConstraintSystem;
use crate::wires::COLUMNS;

impl<F: FftField + SquareRootField> ConstraintSystem<F>
{
    // permutation quotient poly contribution computation
    pub fn perm_quot
    (
        &self,
        lagrange: &WitnessOverDomains<F>,
        oracles: &RandomOracles<F>,
        z: &DensePolynomial<F>,
        alpha: &[F]
    ) -> Result<(Evaluations<F, D<F>>, DensePolynomial<F>), ProofError>
    {
        let l0 = &self.l08.scale(oracles.gamma);

        let (bnd1, res) =
            DenseOrSparsePolynomial::divide_with_q_and_r(&(z - &DensePolynomial::from_coefficients_slice(&[F::one()])).into(),
                &DensePolynomial::from_coefficients_slice(&[-F::one(), F::one()]).into()).
                map_or(Err(ProofError::PolyDivision), |s| Ok(s))?;
        if res.is_zero() == false {return Err(ProofError::PolyDivision)}

        let (bnd2, res) =
            DenseOrSparsePolynomial::divide_with_q_and_r(&(z - &DensePolynomial::from_coefficients_slice(&[F::one()])).into(),
                &DensePolynomial::from_coefficients_slice(&[-self.sid[self.domain.d1.size as usize -3], F::one()]).into()).
                map_or(Err(ProofError::PolyDivision), |s| Ok(s))?;
        if res.is_zero() == false {return Err(ProofError::PolyDivision)}

        Ok((
            &(&lagrange.d8.this.w.iter().zip(self.shift.iter()).
            map(|(p, s)| p + &(l0 + &self.l1.scale(oracles.beta * s))).
            fold(lagrange.d8.this.z.clone(), |x, y| &x * &y)
            -
            &lagrange.d8.this.w.iter().zip(self.sigmal8.iter()).
                map(|(p, s)| p + &(l0 + &s.scale(oracles.beta))).
                fold(lagrange.d8.next.z.clone(), |x, y| &x * &y)).
            scale(oracles.alpha)
            *
            &self.zkpl
            ,
            &bnd1.scale(alpha[0]) + &bnd2.scale(alpha[1])
        ))
    }

    pub fn perm_lnrz
    (
        &self, e: &Vec<ProofEvaluations<F>>,
        z: &DensePolynomial<F>,
        oracles: &RandomOracles<F>,
        alpha: &[F]
    ) -> DensePolynomial<F>
    {
        let scalars = Self::perm_scalars
        (
            e,
            oracles,
            &self.shift,
            alpha,
            self.domain.d1.size,
            self.zkpm.evaluate(oracles.zeta),
            self.sid[self.domain.d1.size as usize -3]
        );
        &z.scale(scalars[0]) + &self.sigmam[COLUMNS-1].scale(scalars[1])
    }

    // permutation linearization poly contribution computation
    pub fn perm_scalars
    (
        e: &Vec<ProofEvaluations<F>>,
        oracles: &RandomOracles<F>,
        shift: &[F; COLUMNS],
        alpha: &[F],
        n: u64,
        z: F,
        w: F,
    ) -> Vec<F>
    {
        let bz = oracles.beta * &oracles.zeta;
        let mut denominator = [oracles.zeta - &F::one(), oracles.zeta - &w];
        algebra::fields::batch_inversion::<F>(&mut denominator);
        let numerator = oracles.zeta.pow(&[n]) - &F::one();

        vec!
        [
            e[0].w.iter().zip(shift.iter()).
                map(|(w, s)| oracles.gamma + &(bz * s) + w).
                fold(oracles.alpha * &z, |x, y| x * y) +
            &(alpha[0] * &numerator * &denominator[0]) +
            &(alpha[1] * &numerator * &denominator[1])
            ,
            -e[0].w.iter().zip(e[0].s.iter()).
                map(|(w, s)| oracles.gamma + &(oracles.beta * s) + w).
                fold(e[1].z * &oracles.beta * &oracles.alpha * &z, |x, y| x * y)
        ]
    }
}
