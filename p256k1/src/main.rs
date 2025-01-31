use bitvec::prelude::*;
use num_traits::{One, Zero};
use p256k1::{
    field,
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
    X: Point,
    r: Scalar,
    V: Point,
}

impl SchnorrProof {
    #[allow(non_snake_case)]
    pub fn new<T: RngCore + CryptoRng>(x: &Scalar, rng: &mut T) -> Self {
        let X = Point::from(x);
        let v = Scalar::random(rng);
        let V = Point::from(&v);
        let mut hasher = Sha256::new();

        hasher.update(G.compress().as_bytes());
        hasher.update(X.compress().as_bytes());
        hasher.update(V.compress().as_bytes());

        let c = hash_to_scalar(&mut hasher);
        let r = v - &c * x;

        SchnorrProof { X, r, V }
    }

    #[allow(non_snake_case)]
    pub fn verify(&self) -> bool {
        let mut hasher = Sha256::new();

        hasher.update(G.compress().as_bytes());
        hasher.update(self.X.compress().as_bytes());
        hasher.update(self.V.compress().as_bytes());

        let c = hash_to_scalar(&mut hasher);

        self.V == &self.r * &G + &c * &self.X
    }
}

#[allow(non_snake_case)]
fn main() {
    let mut rng = OsRng;
    let x = Scalar::random(&mut rng);
    let proof = SchnorrProof::new(&x, &mut rng);
    println!("SchnorrProof verify {}", proof.verify());

    // EC-El Gamal
    
    // message to encrypt
    let mut m = Scalar::random(&mut rng);
    let mut M = Point::new();
    let mut i = 0;
    loop {
	i += 1;
	let fm = field::Element::from(m);
	if let Ok(LM) = Point::lift_x(&fm) {
	    M = LM;
	    break;
	} else {
	    m = Scalar::random(&mut rng);
	}
    }

    println!("took {i} tries to get a valid point {}", M.x());

    // ephemeral key
    let k = Scalar::random(&mut rng);

    // private key
    let x = Scalar::random(&mut rng);

    // public key
    let X = Point::from(x);

    let A = k * G;
    let B = -M + k * X;

    let M1 = x * A - B;

    println!("recovered message                 {}", M1.x());

    // el gamal with field::Element

    // message
    let m = field::Element::random(&mut rng);

    println!("encrypt message   {}", m);

    // generator
    let h = field::Element::random(&mut rng);

    // ephemeral key
    let k = field::Element::random(&mut rng);

    // private key
    let x = field::Element::random(&mut rng);

    // public key
    let y = h^x;

    let A = h^k;
    let B = (m.invert()) * (y^k);

    let m1 = (A^x) * (B.invert());
    println!("decrypted message {}", m1);

    let n1 = -Scalar::one();
    let N = Scalar::from(p256k1::point::N);
    let n12 = -Scalar::one() / Scalar::from(2);
    let n122 = n12 * Scalar::from(2);
    let h = Scalar::random(&mut rng);
    let hn1 = h^n1;
    let hn12 = h^n12;

    println!("N          = {}", hex::encode(N.to_bytes()));
    println!("-1         = {}", hex::encode(n1.to_bytes()));
    println!("-1/2       = {}", hex::encode(n12.to_bytes()));
    println!("2 * (-1/2) = {}", hex::encode(n122.to_bytes()));
    println!("hn1        = {}", hex::encode(hn1.to_bytes()));
    println!("hn12       = {}", hex::encode(hn12.to_bytes()));
    
    
    // el gamal with scalar
    let mut success = 0;
    let mut th_equal_sum = 0;
    let num = 100;//1000;
    for n in 0..num {
	if n % (num/10) == 0 {
	    println!("{}% done", (100*n)/num);
	}
	let m = Scalar::random(&mut rng);

	//println!("encrypt message   {}", m);

	let s4 = Scalar::from(4);
	s4.invert();
	
	// generator
	let h = Scalar::random(&mut rng);

	let hn1 = h^n1;
	
	
	// ephemeral key
	let k = Scalar::random(&mut rng);
	let kbytes = k.to_bytes();

	// private key
	let x = Scalar::random(&mut rng);

	// public key
	let y = h^x;

	let A = h^k;
	let B = (m.invert()) * (y^k);

	let m1 = (A^x) * (B.invert());
	//println!("decrypted message {}", m1);

	// prove that (A,B) encrypts discrete log m of M = m * G
	let M = m * G;

	//println!("B * M     {}", (B * M));
	//println!("(y^k) * G {}", ((y^k) * G));

	let mut hasher = Sha256::new();

	hasher.update(M.compress().as_bytes());
	hasher.update(A.to_bytes());
	hasher.update(B.to_bytes());

	let mut W = Vec::new();
	let mut TH = Vec::new();
	let mut TG = Vec::new();

	for _ in 0..128 {
	    let w = Scalar::random(&mut rng);
	    let th = h^w;
	    let tg = (y^w) * G;

	    hasher.update(th.to_bytes());
	    hasher.update(tg.compress().as_bytes());

	    W.push(w);
	    TH.push(th);
	    TG.push(tg);
	}

	let c = hash_to_scalar(&mut hasher);
	let bytes = c.to_bytes();

	let mut R = Vec::new();
	let mut i = 0;
	for j in 0..bytes.len()/2 {
            let bits = bytes[31 - j].view_bits::<Lsb0>();
            for bit in bits {
		let ci = if *bit { Scalar::one() } else {Scalar::zero() };
		let r = W[i] - (ci * k);

		R.push(r);

		i += 1;
	    }
	}

	// proof consists of (R, c) along with (M, A, B)

	// verify the proof
	let mut hasher = Sha256::new();

	hasher.update(M.compress().as_bytes());
	hasher.update(A.to_bytes());
	hasher.update(B.to_bytes());

	let mut c_plus = 0;
	let mut th_equal = 0;
	let mut tg_equal = 0;
	let mut i = 0;
	for j in 0..bytes.len()/2 {
            let bits = bytes[31 - j].view_bits::<Lsb0>();
            for bit in bits {
		let ci = if *bit { c_plus +=1; Scalar::one() } else {Scalar::zero() };
		let th = (h^R[i]) * (A^ci);
		//let tg = (y^R[i]) * ((Scalar::one() - ci) * G + (ci*B) * M);
		let tg = if *bit {
		    (B * y^R[i]) * M
		} else {
		    (y^R[i]) * G
		};

		hasher.update(th.to_bytes());
		hasher.update(tg.compress().as_bytes());

		//println!("th[{}] = {th}", i);
		//println!("TH[{}] = {}", i, TH[i]);

		if th == TH[i] {
		    th_equal += 1;
		}
		
		//println!("tg[{}] = {tg}", i);
		//println!("TG[{}] = {}", i, TG[i]);
		
		if tg == TG[i] {
		    tg_equal += 1;
		}
		
		i += 1;
	    }

	}

	let c_verify = hash_to_scalar(&mut hasher);
	/*
	println!("c_plus    = {c_plus}");
	println!("c_zero    = {}", (128-c_plus));
	println!("th_equal  = {th_equal}");
	println!("tg_equal  = {tg_equal}");

	println!("c_prove  = {c}");
	println!("c_verify = {c_verify}");
	 */
	if th_equal == 128 {
	    success += 1;
	    println!("success");
	    println!("h = {}", hex::encode(h.to_bytes()));
	    println!("k = {}", hex::encode(k.to_bytes()));
	    println!("x = {}", hex::encode(x.to_bytes()));

	    let mut i = 0;
	    for j in 0..bytes.len()/2 {
		let bits = bytes[31 - j].view_bits::<Lsb0>();
		for bit in bits {
		    if *bit {
			let wbytes = W[i].to_bytes();
			if wbytes < kbytes {
			    println!("w[{i}] = {}", hex::encode(wbytes));
			}
		    }
		    i += 1;
		}
	    }
	} else {
	    let bytes = k.to_bytes();
	    if bytes[0] == 0 {
		println!("weird failure");
		println!("h = {}", hex::encode(h.to_bytes()));
		println!("k = {}", hex::encode(k.to_bytes()));
		println!("x = {}", hex::encode(x.to_bytes()));
	    }
	    let mut i = 0;
	    for j in 0..bytes.len()/2 {
		let bits = bytes[31 - j].view_bits::<Lsb0>();
		for bit in bits {
		    if *bit {
			let wbytes = W[i].to_bytes();
			if wbytes < kbytes {
			    //println!("w[{i}] = {}", hex::encode(wbytes));
			}
		    }
		    i += 1;
		}
	    }
	}
	th_equal_sum += th_equal;
    }

    println!("th_equal avg {}/{}", (th_equal_sum / num), 128);
    println!("success {success}/{num}");
}
