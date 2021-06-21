use anyhow::Result;
use blake3::hash;
use ed25519_dalek_blake3::{Keypair, PublicKey as Pk, SecretKey as Sk};
use rand::rngs::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};

pub fn seed() -> [u8; 32] {
  let mut rng = OsRng {};
  let keypair: Keypair = Keypair::generate(&mut rng);
  keypair.secret.to_bytes()
}

pub fn ed25519_x25519_pk(pk: &Pk) -> PublicKey {
  use curve25519_dalek::edwards::CompressedEdwardsY;
  let cey = CompressedEdwardsY::from_slice(pk.as_bytes());
  let ep = cey.decompress().unwrap();
  PublicKey::from(*ep.to_montgomery().as_bytes())
}

#[test]
fn main() -> Result<()> {
  let a_seed = seed();
  let a_e_sk = Sk::from_bytes(&a_seed)?;
  let a_e_pk: Pk = (&a_e_sk).into();
  let a_x_pk = ed25519_x25519_pk(&a_e_pk);
  let a_x_sk = StaticSecret::from(*hash(&a_seed).as_bytes());

  let b_e_sk = Sk::from_bytes(&seed())?;
  let b_e_pk = Pk::from(&b_e_sk);
  let b_x_pk = ed25519_x25519_pk(&b_e_pk);
  let b_x_sk = StaticSecret::from(*hash(b_e_sk.as_bytes()).as_bytes());

  println!(
    r#"
{:?}

{:?}"#,
    b_x_sk.diffie_hellman(&a_x_pk).as_bytes(),
    a_x_sk.diffie_hellman(&b_x_pk).as_bytes(),
  );

  Ok(())
}

/*
let ed25519 = {
  let sk = Sk::from_bytes(&seed())?;
  let mut skv = sk.as_bytes().to_vec();
  skv.extend_from_slice(Pk::from(&sk).as_bytes());
  Keypair::from_bytes(&skv as &[u8])?
};
dbg!(ed25519);
*/
