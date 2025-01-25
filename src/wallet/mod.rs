use p256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};
use rand_core::OsRng;
use ripemd160::{Digest as RipDigest, Ripemd160};
use serde::Serialize;
use sha2::{Digest, Sha256};

pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    address: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct WalletTransaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub public_key: String,
    pub signature: String,
}

impl Wallet {
    pub fn new() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key().clone();
        let mut address = String::new();
        let mut gen_address = || {
            let key_points = verifying_key.to_encoded_point(false);

            if let (Some(x), Some(y)) = (key_points.x(), key_points.y()) {
                let mut pub_key_bytes = Vec::with_capacity(x.len() + y.len());
                pub_key_bytes.extend_from_slice(x);
                pub_key_bytes.extend_from_slice(y);
                let hash = Sha256::digest(&pub_key_bytes);
                let mut hasher = Ripemd160::new();
                hasher.update(&hash);
                let mut hash_result = hasher.finalize().to_vec();
                hash_result.insert(0, 0x00);
                let hash2 = Sha256::digest(&hash_result);
                let hash3 = Sha256::digest(&hash2);
                let checksum = &hash[0..4];
                let full_hash = [hash_result, checksum.to_vec()].concat();
                address = bs58::encode(full_hash).into_string();
            }
        };

        gen_address();

        Wallet {
            signing_key,
            verifying_key,
            address,
        }
    }

    pub fn private_key_str(&self) -> String {
        //convert the private key into hex string
        hex::encode(self.signing_key.to_bytes())
    }

    pub fn public_key_str(&self) -> String {
        let key_points = self.verifying_key.to_encoded_point(false);
        if let (Some(x), Some(y)) = (key_points.x(), key_points.y()) {
            let pub_str = hex::encode(x) + hex::encode(y).as_str();
            pub_str
        } else {
            String::new()
        }
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub fn sign_transaction(&self, reciever: &String, amount: u64) -> WalletTransaction {
        let mut transaction = WalletTransaction {
            sender: self.address.clone(),
            recipient: reciever.clone(),
            amount,
            signature: String::new(),
            public_key: self.public_key_str(),
        };

        let serialized_str = serde_json::to_string(&transaction).unwrap();
        let serialized = serialized_str.as_bytes();
        let sig: Signature = self.signing_key.sign(serialized);
        transaction.signature = hex::encode(sig.to_bytes());
        transaction
    }

    pub fn verify_transaction(transaction: &WalletTransaction) -> bool {
        let signature_str = transaction.signature.clone();
        let signature_bin = hex::decode(signature_str).unwrap();
        let mut transaction_clone = transaction.clone();
        transaction_clone.signature = String::new();

        let serialized_str = serde_json::to_string(&transaction_clone).unwrap();
        let serialized = serialized_str.as_bytes();

        let sig_array: [u8; 64] = signature_bin.try_into().unwrap();

        let signature = match Signature::from_bytes(&sig_array.into()) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        let public_key_str = transaction_clone.public_key.clone();
        let mut public_key_bin = hex::decode(public_key_str).unwrap();
        public_key_bin.insert(0, 0x04);
        let public_key = VerifyingKey::from_sec1_bytes(&public_key_bin).unwrap();
        public_key.verify(serialized, &signature).is_ok()
    }
}
