use hex;
use num_bigint::BigUint;
use zkp_course::*;

#[test]
fn test_exponentiate() {
    let alpha = BigUint::from(4u32);
    let beta = BigUint::from(9u32);
    let p = BigUint::from(23u32);
    let q = BigUint::from(11u32);
    let x = BigUint::from(6u32);
    let k = BigUint::from(7u32);
    let c = BigUint::from(4u32);
    let zkp = ZKP { p, q, alpha, beta };
    let y1 = zkp.exponentiate(&zkp.alpha, &x);
    let y2 = zkp.exponentiate(&zkp.beta, &x);

    assert_eq!(y1, BigUint::from(2u32));
    assert_eq!(y2, BigUint::from(3u32));

    let r1 = zkp.exponentiate(&zkp.alpha, &k);
    let r2 = zkp.exponentiate(&zkp.beta, &k);
    assert_eq!(r1, BigUint::from(8u32));
    assert_eq!(r2, BigUint::from(4u32));

    let s = zkp.solve(&k, &c, &x);
    assert_eq!(s, BigUint::from(5u32));

    let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
    assert!(result);
}

#[test]
fn test_exponentiate_random() {
    let alpha = BigUint::from(4u32);
    let beta = BigUint::from(9u32);
    let p = BigUint::from(23u32);
    let q = BigUint::from(11u32);
    let x = BigUint::from(6u32);
    let zkp = ZKP { alpha, beta, q, p };
    let k = generate_random_below(&zkp.q);
    let c = generate_random_below(&zkp.q);

    let y1 = zkp.exponentiate(&zkp.alpha, &x);
    let y2 = zkp.exponentiate(&zkp.beta, &x);

    let r1 = zkp.exponentiate(&zkp.alpha, &k);
    let r2 = zkp.exponentiate(&zkp.beta, &k);

    let s = zkp.solve(&k, &c, &x);

    let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
    assert!(result);
}

#[test]
fn test_1024bits_consts() {
    // https://www.rfc-editor.org/rfc/rfc5114.html
    // The hexadecimal value of the prime is:
    let p_hex = hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371",).unwrap();

    // The generator generates a prime-order subgroup of size:
    let q_hex = hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap();

    // The hexadecimal value of the generator is:
    let alpha_hex = hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5",    )    .unwrap();

    let p = BigUint::from_bytes_be(&p_hex);
    let q = BigUint::from_bytes_be(&q_hex);
    let x = generate_random_below(&q);
    let alpha = BigUint::from_bytes_be(&alpha_hex);
    // alpha**x is also a generator
    let beta = alpha.modpow(&generate_random_below(&q), &p);
    let zkp = ZKP { alpha, beta, q, p };
    let k = generate_random_below(&zkp.q);
    let c = generate_random_below(&zkp.q);

    let y1 = zkp.exponentiate(&zkp.alpha, &x);
    let y2 = zkp.exponentiate(&zkp.beta, &x);

    let r1 = zkp.exponentiate(&zkp.alpha, &k);
    let r2 = zkp.exponentiate(&zkp.beta, &k);

    let s = zkp.solve(&k, &c, &x);

    let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
    assert!(result);
}
