use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::short_weierstrass::{
        curves::bls12_381::curve::BLS12381Curve, point::ShortWeierstrassProjectivePoint,
    },
    elliptic_curve::traits::IsEllipticCurve,
    traits::{AsBytes, ByteConversion},
    unsigned_integer::element::U256,
};

///  compute the public key from a given secret key
/// on the BLS12_381 curve. It uses elliptic curve multiplication
/// of the secret key with the generator point to derive the public key.
pub fn calculate_public_key(secret_key: U256) -> ShortWeierstrassProjectivePoint<BLS12381Curve> {
    let generator = BLS12381Curve::generator();
    let public_key = generator.operate_with_self(secret_key);
    public_key
}

fn main() {
    let secret_key = U256::from_hex_unchecked("6C616D6264617370");
    let public_key = calculate_public_key(secret_key);
    let public_key_bytes = public_key.as_bytes();
    let public_key_u256 =
        U256::from_bytes_be(&public_key_bytes).expect("Failed to convert public key to U256");

    println!("Derived Public Key: {:?}", public_key_u256.to_hex());
}
