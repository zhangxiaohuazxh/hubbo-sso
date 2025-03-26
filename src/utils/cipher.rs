use anyhow::{Context, Result};
use base64::prelude::*;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
use rsa::pkcs1v15::{Signature, SigningKey, VerifyingKey};
use rsa::signature::{RandomizedSigner, SignatureEncoding, SignerMut, Verifier};
use rsa::{
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
    pkcs8::{DecodePrivateKey, DecodePublicKey},
};
use sha2::Sha256;

type Aes192CbcEnc = cbc::Encryptor<aes::Aes192>;
type Aes192CbcDec = cbc::Decryptor<aes::Aes192>;

/// ## 不要尝试使用下面的rsa密钥,因为我已经删除了中间的部分数据
const PRIVATE_KEY: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQCU5eroItCu2xibuPtB
4Rc1Cs4zzyeZuNXJAHK1n9gJBLMP2AIFZkVi6e6nwQKBgQC157O/k+JUp2lxuAYg
qrtYw99dUJuSlDcF8Shr620ygndRUP0PYRPkpwlb2S0YkDf+HUYG3VKIVrl9OZFz
j8Lisp6uwGCqDusOLd62QekjTuMdsm5EzsenuqJ+p0RSsnBuOIRIYeL7q+RBHptB
D3OyMByjRql/0GjvXjFDespjKA==
-----END PRIVATE KEY-----
"#;

const PUBLIC_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAmhw4X8nXkUAN920BjwdX
ywIDAQAB
-----END PUBLIC KEY-----
"#;

/// ## 使用公钥对源数据进行加密
/// ### @param 源数据
/// ### @return base64编码的加密数据
#[inline]
pub async fn encrypt(data: &[u8]) -> Result<String> {
    let public_key = RsaPublicKey::from_public_key_pem(PUBLIC_KEY)?;
    // todo 这里是一个大坑,浪费了我一天的时间,最后才发现是rand库版本的问题,非0.8版本的thread_rng有问题,后续再调整,高版本已经废弃这个api
    let mut rng = rand::thread_rng();
    Ok(BASE64_STANDARD.encode(public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data)?))
}

/// 使用私钥解密rsa公钥加密过的数据
/// ### @param encoding_data 密文
/// ### @return 原始数据
#[inline]
pub async fn decrypt(encoding_data: String) -> Result<Vec<u8>> {
    let data = BASE64_STANDARD
        .decode(&encoding_data)
        .with_context(|| format!("解码 {} 数据失败", encoding_data));
    let private_key = RsaPrivateKey::from_pkcs8_pem(PRIVATE_KEY).unwrap();
    Ok(private_key.decrypt(Pkcs1v15Encrypt, &data.unwrap())?)
}

/// ### rsa签名
/// ### @param 源数据
/// ### @return base64编码后的签名
#[inline]
pub async fn sign(data: &[u8]) -> Result<String> {
    let private_key = RsaPrivateKey::from_pkcs8_pem(PRIVATE_KEY)?;
    let sign_key: SigningKey<Sha256> = SigningKey::new_unprefixed(private_key);
    let mut rng = rand::thread_rng();
    Ok(BASE64_STANDARD.encode(sign_key.sign_with_rng(&mut rng, data).to_vec()))
}

/// ## 使用公钥进行rsa验签
/// ### @param data 源数据
/// ### @param sign 签名
/// ### panic 验签失败
#[inline]
pub async fn verify(data: &[u8], sign: &[u8]) -> Result<()> {
    let public_key = RsaPublicKey::from_public_key_pem(PUBLIC_KEY)?;
    let verifying_key = VerifyingKey::<Sha256>::new_unprefixed(public_key);
    let signature: Signature = Signature::try_from(sign)?;
    verifying_key.verify(data, &signature)?;
    Ok(())
}

pub async fn aes_decrypt(data: &mut [u8]) -> Result<String> {
    // 三种不同的密钥长度：16字节对应AES128、24字节对应AES192和32字节对应AES256,支付宝是AES192,不支持其他的模式
    let key = b"==";
    let iv = [0x24; 16];
    let cipher = Aes192CbcEnc::new_from_slices(key, &iv)?;
    let res = cipher
        .encrypt_padded_mut::<Pkcs7>(data, data.len())
        .unwrap();
    Ok("abc".to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[actix_rt::test]
    async fn test_encrypt() -> Result<()> {
        let res = encrypt("abc".as_bytes()).await?;
        println!("test_encrypt result {:#?}", res);
        Ok(())
    }

    #[actix_rt::test]
    async fn test_decrypt() -> Result<()> {
        let res = String::from_utf8(decrypt(encrypt(b"abc").await?).await?)?;
        println!("test_decrypt result {:#?}", res);
        Ok(())
    }

    #[actix_rt::test]
    async fn test_sign() -> Result<()> {
        let res = sign(b"abc").await?;
        println!("test_sign result {:#?}", res);
        Ok(())
    }

    #[actix_rt::test]
    async fn test_verify_sign() -> Result<()> {
        verify(
            b"abc",
            &BASE64_STANDARD.decode(crate::utils::cipher::sign(b"abc").await?)?,
        )
        .await?;
        println!("第一轮验签通过");
        verify(b"abc", sign(b"abc").await?.as_bytes()).await?;
        Ok(())
    }

    #[actix_rt::test]
    async fn test_aes_decrypt() -> Result<()> {
        let mut data = [97, 98, 99];
        let result = aes_decrypt(&mut data).await?;
        println!("test_aes_decrypt result {:#?}", result);
        Ok(())
    }
}
