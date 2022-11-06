use sha3::{
    Digest, Sha3_256, 
};

use secp256k1_math::{
    scalar::Scalar,
    point::Point,
};

fn hash_to_scalar(hasher: &mut Sha3_256) -> Scalar {
    let h = hasher.clone();
    let hash = h.finalize();
    let mut hash_bytes: [u8; 32] = [0; 32];
    hash_bytes.clone_from_slice(hash.as_slice());
    
    Scalar::from(hash_bytes)
}

#[allow(non_snake_case)]
struct SchnorrProof {
    X: Point,
    r: Scalar,
    V: Point,
}

impl SchnorrProof {
    #[allow(non_snake_case)]
    pub fn new(x: &Scalar) -> Self {
        let mut G = Point::G();
        let mut X = Point::from(x);
        let v = Scalar::from(12345);
        let mut V = Point::from(&v);
        let mut hasher = Sha3_256::new();

        hasher.update(G.compress().as_bytes());
        hasher.update(X.compress().as_bytes());
        hasher.update(V.compress().as_bytes());

        let c = hash_to_scalar(&mut hasher);
        let r = &v - &(&c * x);
        
        SchnorrProof{
            X: X,
            r: r,
            V: V,
        }
    }

    #[allow(non_snake_case)]
    pub fn verify(&mut self) -> bool {
        let mut G = Point::G();
        let mut hasher = Sha3_256::new();

        hasher.update(G.compress().as_bytes());
        hasher.update(self.X.compress().as_bytes());
        hasher.update(self.V.compress().as_bytes());

        let c = hash_to_scalar(&mut hasher);
        
        self.V == &self.r * &G + &c * &self.X
    }
}

#[allow(non_snake_case)]
fn main() {
    let x = Scalar::from(12345);
    let mut proof = SchnorrProof::new(&x);
    println!("SchnorrProof verify {}", proof.verify());
}
