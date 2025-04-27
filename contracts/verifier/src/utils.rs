use crate::error::ContractError;
use crate::state::{BaseProof, ExtraProof};
use crate::types::{BaseField, Groth16Proof, ScalarField};
use cosmwasm_std::{ensure, ensure_eq, Uint256};
use pairing_ce::bn256::{Bn256, Fq2, G1Affine, G2Affine, G1, G2};
use pairing_ce::ff::PrimeField;
use pairing_ce::{CurveAffine, CurveProjective, Engine};

pub fn u256_to_base_field(x: &Uint256) -> BaseField {
    BaseField::from_str(&x.to_string()).unwrap()
}

pub fn u256_to_scalar_field(x: &Uint256) -> ScalarField {
    ScalarField::from_str(&x.to_string()).unwrap()
}

pub fn g1_from_xy(x: &Uint256, y: &Uint256) -> Result<G1, ContractError> {
    match G1Affine::from_xy_checked(u256_to_base_field(x), u256_to_base_field(y)) {
        Ok(g1) => Ok(g1.into()),
        Err(_) => Err(ContractError::PointNotOnCurve),
    }
}

pub fn g2_from_xy(
    x_real: &Uint256,
    x_img: &Uint256,
    y_real: &Uint256,
    y_img: &Uint256,
) -> Result<G2, ContractError> {
    match G2Affine::from_xy_checked(
        Fq2 {
            c0: u256_to_base_field(x_real),
            c1: u256_to_base_field(x_img),
        },
        Fq2 {
            c0: u256_to_base_field(y_real),
            c1: u256_to_base_field(y_img),
        },
    ) {
        Ok(g2) => Ok(g2.into()),
        Err(_) => Err(ContractError::PointNotOnCurve),
    }
}

pub fn verify_proof(
    base_proof: &BaseProof,
    extra_proof: &ExtraProof,
    groth16_proof: &Groth16Proof,
    pub_signals: &Vec<&Uint256>,
) -> Result<(), ContractError> {
    let BaseProof {
        r,
        q,
        alphax,
        alphay,
        betax1,
        betax2,
        betay1,
        betay2,
        gammax1,
        gammax2,
        gammay1,
        gammay2,
    } = base_proof;
    let ExtraProof {
        deltax1,
        deltax2,
        deltay1,
        deltay2,
        ic_x,
        ic_y,
    } = extra_proof;
    let Groth16Proof { pi_a, pi_b, pi_c } = groth16_proof;
    // Validate that all evaluations ∈ F
    for x in pub_signals {
        ensure!(*x < q, ContractError::NotInBaseField(**x));
    }

    // Check pairing
    // Compute the linear combination vk_x
    ensure_eq!(
        ic_x.len(),
        pub_signals.len() + 1,
        ContractError::MalformedProof
    );
    ensure_eq!(ic_x.len(), ic_y.len(), ContractError::MalformedProof);

    let mut vk_x = g1_from_xy(&ic_x[0], &ic_y[0])?;
    for i in 1..ic_x.len() {
        let tmp = g1_from_xy(&ic_x[i], &ic_y[i])?.into_affine();
        vk_x.add_assign(&tmp.mul(u256_to_scalar_field(pub_signals[i - 1])));
    }

    // compute
    let a = g1_from_xy(&pi_a[0], &pi_a[1])?;
    let b = g2_from_xy(&pi_b[0][0], &pi_b[0][1], &pi_b[1][0], &pi_b[1][1])?;
    let alpha = g1_from_xy(alphax, alphay)?;
    let beta = g2_from_xy(betax2, betax1, betay2, betay1)?;
    let mut gamma_neg = g2_from_xy(gammax2, gammax1, gammay2, gammay1)?;
    gamma_neg.negate();
    let c = g1_from_xy(&pi_c[0], &pi_c[1])?;
    let mut delta_neg = g2_from_xy(deltax2, deltax1, deltay2, deltay1)?;
    delta_neg.negate();

    // A * B + inputs * (-gamma) + C * (-delta) = alpha * beta
    if Bn256::final_exponentiation(&Bn256::miller_loop([
        &(&a.into_affine().prepare(), &b.into_affine().prepare()),
        &(
            &vk_x.into_affine().prepare(),
            &gamma_neg.into_affine().prepare(),
        ),
        &(
            &c.into_affine().prepare(),
            &delta_neg.into_affine().prepare(),
        ),
    ]))
    .unwrap()
        == Bn256::pairing(alpha.into_affine(), beta.into_affine())
    {
        Ok(())
    } else {
        Err(ContractError::PairingFailed)
    }
}
