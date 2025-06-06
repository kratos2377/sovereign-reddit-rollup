use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rand::Rng;
use rand::distributions::Alphanumeric;
use sov_modules_api::{digest::Digest, CryptoSpec, Spec};


pub fn get_user_address<S: Spec>(
    name: &str,
    sender: &[u8],
) -> S::Address {
    let mut hasher = <S::CryptoSpec as CryptoSpec>::Hasher::new();
    hasher.update(sender);
    hasher.update(name.as_bytes());

    let hash: [u8; 32] = hasher.finalize().into();
S::Address::from(hash.into())
}


pub fn get_sub_address<S: Spec>(
    subname: &str
) -> S::Address {
    let mut hasher = <S::CryptoSpec as CryptoSpec>::Hasher::new();

    hasher.update(subname.as_bytes());

    let hash: [u8; 32] = hasher.finalize().into();
    S::Address::from(hash.into())
}


pub fn get_post_address<S: Spec>(
    user_address: &[u8],
    sub_address: &[u8],
) -> S::Address {
    let random_string: String= rand::thread_rng().sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();
    let mut hasher = <S::CryptoSpec as CryptoSpec>::Hasher::new();
    
    hasher.update(user_address);
    hasher.update(sub_address);
    hasher.update(random_string.as_bytes());

    let hash: [u8; 32] = hasher.finalize().into();
    S::Address::from(hash.into())
}