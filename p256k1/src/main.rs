use p256k1::{
    point::{Point, G},
    scalar::Scalar,
};
use rand_core::{CryptoRng, OsRng, RngCore};
use sha2::{Digest, Sha256};

fn hash_to_scalar(hasher: &mut Sha256) -> Scalar {
    let h = hasher.clone();
    let hash = h.finalize();
    let mut hash_bytes: [u8; 32] = [0; 32];
    hash_bytes.clone_from_slice(hash.as_slice());

    Scalar::from(hash_bytes)
}

#[allow(non_snake_case)]
struct SchnorrProof {
    R: Point,
    s: Scalar,
}

impl SchnorrProof {
    #[allow(non_snake_case)]
    pub fn new<T: RngCore + CryptoRng>(x: &Scalar, rng: &mut T) -> Self {
        let X = Point::from(x);
        let r = Scalar::random(rng);
        let R = Point::from(&r);
        let c = Self::challenge(&X, &R);
        let s = r - &c * x;

        SchnorrProof { R, s }
    }

    #[allow(non_snake_case)]
    pub fn verify(&self, X: Point) -> bool {
        let c = Self::challenge(&X, &self.R);
        self.R == &self.s * &G + &c * &X
    }

    #[allow(non_snake_case)]
    pub fn challenge(X: &Point, R: &Point) -> Scalar {
        let mut hasher = Sha256::new();

        hasher.update(G.compress().as_bytes());
        hasher.update(X.compress().as_bytes());
        hasher.update(R.compress().as_bytes());

        hash_to_scalar(&mut hasher)
    }
}

#[allow(non_snake_case)]
fn main() {
    let mut rng = OsRng;
    let x = Scalar::random(&mut rng);
    let proof = SchnorrProof::new(&x, &mut rng);
    println!("SchnorrProof verify {}", proof.verify(x * G));
}
