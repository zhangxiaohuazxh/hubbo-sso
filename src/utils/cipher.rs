use anyhow::{Context, Result};
use base64::prelude::*;
use cbc::cipher::{BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
use rsa::{
    Pkcs1v15Encrypt, Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey,
    pkcs8::{DecodePrivateKey, DecodePublicKey},
};
use sha2::{Digest, Sha256};

type Aes192CbcEnc = cbc::Encryptor<aes::Aes192>;
#[allow(unused)]
type Aes192CbcDec = cbc::Decryptor<aes::Aes192>;

/// ## 不要尝试使用下面的rsa密钥,因为我已经删除了中间的部分数据
const PRIVATE_KEY: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQCU5eroItCu2xib
D3OyMByjRql/0GjvXjFDespjKA==
-----END PRIVATE KEY-----
"#;

const PUBLIC_KEY: &str = "";

const ALI_PUBLIC_KEY: &str = r#"M+PzqS3KdKCCphazkYe6+Ao7AgkDcf76SFeFJZIcF/CFkvNnLEsLKCYmLUn8lKi9yevMLRErP30z9DP5BfZFMQIDAQAB"#;

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
    Ok(BASE64_STANDARD
        .encode(private_key.sign(rsa::Pkcs1v15Sign::new::<Sha256>(), &Sha256::digest(data))?))
}

/// ## 使用公钥进行rsa验签
/// ### @param data 源数据
/// ### @param sign 签名
/// ### panic 验签失败
#[inline]
pub async fn verify(data: &[u8], sign: &str) -> Result<()> {
    let public_key = RsaPublicKey::from_public_key_der(&BASE64_STANDARD.decode(ALI_PUBLIC_KEY)?)?;
    public_key.verify(
        Pkcs1v15Sign::new::<Sha256>(),
        &Sha256::digest(data),
        &BASE64_STANDARD.decode(sign)?,
    )?;
    Ok(())
}

pub async fn aes_decrypt(data: &mut [u8]) -> Result<String> {
    // 三种不同的密钥长度：16字节对应AES128、24字节对应AES192和32字节对应AES256,支付宝是AES192,不支持其他的模式
    let key = b"==";
    let iv = [0x24; 16];
    let cipher = Aes192CbcEnc::new_from_slices(key, &iv)?;
    let _ = cipher
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
            br#"access_token=&auth_start=2025-03-23 18:59:42&expires_in=1296000&re_expires_in=2592000&refresh_token=&open_id="#,
            r#"VnnSCXT5XivGj+0++FPr2eZaXgfyM12HaOuk9tYc//XfDT0FjdYZNBFUWrhK29NglXrMT1/gVY3N28psDYzACmMnR1fx759HkYhcB4ENJ//lDnrJpcxW3b5jO6skTniR+ys++K61vBAhDcfABLNRJ+wV5zUsWrp9gVPrm8G1G/ofUxCD2Kj/NDOQCIw=="#
        )
        .await?;
        println!("第一轮验签通过");
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
