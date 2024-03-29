use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::field_extension::BLS12381PrimeField;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381;
use lambdaworks_math::{cyclic_group::IsGroup, elliptic_curve::traits::IsEllipticCurve, field::element::FieldElement};

// secret key -> 0x6C616D6264617370
type FEE = FieldElement<BLS12381PrimeField>;

fn main() {
    let gen = bls12_381::curve::BLS12381Curve::generator();
    let secret_key = 7809643498195481456u64;
    let public_key = gen.operate_with_self(secret_key);

    let public_key_affine = public_key.to_affine();
    let public_key_x = public_key_affine.x();
    let public_key_y = public_key_affine.y();
    println!("Public key x coordinate: {}", public_key_x);
    println!("Public key y coordinate: {}", public_key_y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_key_derivation() {
        let g = bls12_381::curve::BLS12381Curve::generator();
        let secret_key = 7809643498195481456u64;
        let public_key = g.operate_with_self(secret_key);

        let res_x = FEE::new_base("67F9FFC5EAF6C19292112EADF50C11E7460E7568493141676F6BA1374BADD9F6AB1F2F5E155B0E3D2F4C1DE79554F80");
        let res_y = FEE::new_base("18509D22F2107B667A8F75DE737A4FB967F6C3E745A7C2361868515402318F006BD360B8A8763D7844381C6E510799CC");
        let expected_pk = bls12_381::curve::BLS12381Curve::create_point_from_affine(res_x, res_y).unwrap();

        assert_eq!(expected_pk, public_key, "this ain't it man");
    }
}
