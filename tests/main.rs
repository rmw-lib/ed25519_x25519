use anyhow::Result;
use ed25519_dalek_blake3::{Keypair, PublicKey, SecretKey};
use rand::rngs::OsRng;

pub fn seed() -> [u8; 32] {
  let mut rng = OsRng {};
  let keypair: Keypair = Keypair::generate(&mut rng);
  keypair.secret.to_bytes()
}

#[test]
fn main() -> Result<()> {
  /*
  let ed25519_a = {
    let sk = SecretKey::from_bytes(&seed())?;
    let mut skv = sk.as_bytes().to_vec();
    skv.extend_from_slice(PublicKey::from(&sk).as_bytes());
    Keypair::from_bytes(&skv as &[u8])?
  };
  dbg!(ed25519);
  */

  let a_sk = SecretKey::from_bytes(&seed())?;
  let a_pk = PublicKey::from(&a_sk);
  println!(
    r#"a
sk {:?}
pk {:?}"#,
    a_sk.as_bytes(),
    a_pk.as_bytes()
  );

  let b_sk = SecretKey::from_bytes(&seed())?;
  let b_pk = PublicKey::from(&b_sk);
  println!(
    r#"b
sk {:?}
pk {:?}"#,
    b_sk.as_bytes(),
    b_pk.as_bytes()
  );

  Ok(())
}
