use rand_chacha::rand_core::SeedableRng;
use rsa::pkcs1v15::{Signature, SigningKey, VerifyingKey};
use rsa::signature::{RandomizedSigner, SignatureEncoding, SignerMut, Verifier};
use rsa::{
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
    pkcs8::{DecodePrivateKey, DecodePublicKey},
};
use sha2::Sha256;

/// ## 不要尝试使用下面的rsa密钥,因为我已经删除了中间的部分数据
const PRIVATE_KEY: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCaHDhfydeRQA33
bQGPB1dIXzH7lPjPAxk0D8DC2ZzuNUt2UMX1EZEhYcyjpDbGk2ISCCXSWfEnq7wR
-----END PRIVATE KEY-----
"#;

const PUBLIC_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAmhw4X8nXkUAN920BjwdX
-----END PUBLIC KEY-----
"#;

#[inline]
pub async fn encrypt(data: &[u8]) -> Vec<u8> {
    let public_key = RsaPublicKey::from_public_key_pem(PUBLIC_KEY).unwrap();
    // todo 这里是一个大坑,浪费了我一天的时间,最后才发现是rand库版本的问题,非0.8版本的thread_rng有问题,后续再调整,高版本已经废弃这个api
    let mut rng = rand::thread_rng();
    public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data).unwrap()
}

#[inline]
pub async fn sign(data: &[u8]) -> Vec<u8> {
    let private_key = RsaPrivateKey::from_pkcs8_pem(PRIVATE_KEY).unwrap();
    let sign_key: SigningKey<Sha256> = SigningKey::new_unprefixed(private_key);
    let mut rng = rand::thread_rng();
    let signature = sign_key.sign_with_rng(&mut rng, data);
    signature.to_vec()
}

#[inline]
pub async fn decrypt(encoding_data: &[u8]) -> Vec<u8> {
    let private_key = RsaPrivateKey::from_pkcs8_pem(PRIVATE_KEY).unwrap();
    private_key.decrypt(Pkcs1v15Encrypt, encoding_data).unwrap()
}

#[inline]
pub async fn verify(data: &[u8], sign: &[u8]) -> anyhow::Result<()> {
    let public_key = RsaPublicKey::from_public_key_pem(PUBLIC_KEY)?;
    let verifying_key = VerifyingKey::<Sha256>::new_unprefixed(public_key);
    let signature: Signature = Signature::try_from(sign)?;
    verifying_key.verify(data, &signature)?;
    Ok(())
}
#[cfg(test)]
mod test {
    use super::*;

    #[actix_rt::test]
    async fn test_encrypt() {
        let res = encrypt("".as_bytes()).await;
        println!("test_encrypt result {:#?}", res);
    }

    #[actix_rt::test]
    async fn test_decrypt() {
        let res = decrypt(&encrypt(b"abc").await).await;
        println!("test_decrypt result {:#?}", res);
    }

    #[actix_rt::test]
    async fn test_sign() {
        let res = sign(b"abc").await;
        println!("test_sign result {:#?}", res);
    }

    #[actix_rt::test]
    async fn test_verify_sign() {
        let _ = verify(b"abc", &sign(b"abc").await).await;
        // let _ = verify(b"abc", b"bbc").await;
    }
}
