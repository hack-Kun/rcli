// 文本签名和验证
use crate::cli::TextFormat;
use crate::tool::get_reader;
use anyhow::Ok;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{collections::HashMap, fs, io::Read, str::FromStr};

// black3 签名方式
pub struct Black3Singer {
    // key: Vec<u8>,
    key: String,
}

impl Black3Singer {
    // 通过直接传入key构建Black3Singer
    pub fn new(key: String) -> Self {
        Self { key }
    }

    // 通过获取文件中的key构建Black3Singer
    fn try_new(path: String) -> anyhow::Result<Self> {
        let mut read = get_reader(&path)?;
        let mut key = Vec::new();
        read.read_to_end(&mut key)?;
        let key = String::from_utf8(key)?;
        Ok(Self::new(key))
    }
    // 生成key
    fn generate_key() -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
        let key = super::generate(32, false, false, false, false)?;
        Ok(HashMap::from([("black3.txt", key.as_bytes().into())]))
    }
}

// black3 签名实现
impl Encryption for Black3Singer {
    /// read 是用来读取要加密的文件，返回签名字符串
    fn sign(&self, read: &mut dyn Read) -> anyhow::Result<String> {
        let mut buf: Vec<u8> = Vec::new();
        read.read_to_end(&mut buf)?;
        // 计算hash
        let mut hash = blake3::Hasher::new_derive_key(&self.key);
        hash.update(buf.as_slice());
        let res = hash.finalize();
        println!("{}", res);
        Ok(res.to_string())
    }
}
// blask3 验证实现
impl Verify for Black3Singer {
    /// read 是用来读取要验证的文件，返回是否验证成功
    fn verify_for_file(&self, read: &mut dyn Read, sign: String) -> anyhow::Result<bool> {
        // 去取文件
        let mut buf = Vec::new();
        read.read_to_end(&mut buf)?;
        // 计算hash
        let mut hash = blake3::Hasher::new_derive_key(&self.key);
        hash.update(&buf);
        let res = hash.finalize();
        // 与hash签名对比
        if res.to_string().eq(&sign.trim()) {
            println!("签名验证成功");
            Ok(true)
        } else {
            println!("签名验证失败");
            Ok(false)
        }
    }
}

// ed25519 签名方式
pub struct Ed25519Singer {
    /// 私钥，用来做签名
    key: SigningKey,
}

pub struct Ed25519Verifier {
    /// 公钥，用来做验证
    key: VerifyingKey,
}

// 实现签名的方法
impl Encryption for Ed25519Singer {
    // read 是用来读取要加密的文件，返回签名字符串
    fn sign(&self, read: &mut dyn Read) -> anyhow::Result<String> {
        let mut buf = Vec::new();
        read.read_to_end(&mut buf)?;
        // 签名文件
        let signature = self.key.sign(&buf);
        Ok(signature.to_string())
    }
}

// 实现验证的方法
impl Verify for Ed25519Verifier {
    // read 是用来读取要验证的文件，返回是否验证成功, sign 则为要验证的签名字符串
    fn verify_for_file(&self, read: &mut dyn Read, sign: String) -> anyhow::Result<bool> {
        // 读取文件
        let mut buf = Vec::new();
        read.read_to_end(&mut buf)?;
        // 通过私钥进行签名验证
        let signature = Signature::from_str(&sign)?;
        let res = self.key.verify(buf.as_slice(), &signature).is_ok();
        if res {
            println!("签名验证成功");
            Ok(true)
        } else {
            println!("签名验证失败");
            Ok(false)
        }
    }
}

// 构建密钥
impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }
    fn try_new(path: String) -> anyhow::Result<Self> {
        let mut read = get_reader(&path)?;
        let mut key = Vec::new();
        read.read_to_end(&mut key)?;
        let key = key.as_slice();
        let key = key[..32].try_into()?;
        Ok(Self::new(key))
    }
}
// 构建密钥及生成密钥
impl Ed25519Singer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }
    fn try_new(path: String) -> anyhow::Result<Self> {
        let mut read = get_reader(&path)?;
        let mut key = Vec::new();
        read.read_to_end(&mut key)?;
        let key = key.as_slice();
        let key = key[..32].try_into()?;
        Ok(Self::new(SigningKey::from_bytes(key)))
    }

    fn generate_key() -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key();
        let mut map = HashMap::new();

        map.insert("ed25519.sk", sk.as_bytes().into());
        map.insert("ed25519.pk", pk.as_bytes().into());
        Ok(map)
    }
}

/// input 用户要加密内容
pub fn sign(key: String, input: String, format: TextFormat) -> anyhow::Result<String> {
    // 获取一个输入流，读取用户要加密的数据
    let mut read = get_reader(&input)?;
    let res = match format {
        TextFormat::Black3 => Black3Singer::try_new(key)?.sign(&mut read)?,
        TextFormat::Ed25519 => Ed25519Singer::try_new(key)?.sign(&mut read)?,
    };

    Ok(res)
}

// 生成密钥api
pub fn generate_key(format: TextFormat, output: String) -> anyhow::Result<()> {
    let map = match format {
        TextFormat::Black3 => Black3Singer::generate_key()?,
        TextFormat::Ed25519 => Ed25519Singer::generate_key()?,
    };
    for (k, v) in map.iter() {
        let output = output.clone() + "/" + k;
        println!("{}", output);
        fs::write(output, v)?;
    }

    Ok(())
}

// 解密api实现
pub fn verify(key: String, input: String, format: TextFormat, sign: String) -> anyhow::Result<()> {
    let mut read = get_reader(&input)?;
    match format {
        TextFormat::Black3 => {
            Black3Singer::try_new(key)?.verify_for_file(&mut read, sign)?;
        }
        TextFormat::Ed25519 => {
            Ed25519Verifier::try_new(key)?.verify_for_file(&mut read, sign)?;
        }
    }

    Ok(())
}

// 签名和验证接口
pub trait Encryption {
    fn sign(&self, read: &mut dyn Read) -> anyhow::Result<String>;
}

pub trait Verify {
    fn verify_for_file(&self, read: &mut dyn Read, sign: String) -> anyhow::Result<bool>;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sign_black3() {
        let key = "fixtures/black3.txt".to_string();
        let input = "fixtures/b64.txt".to_string();
        let format = TextFormat::Black3;
        let res = sign(key, input, format);
        println!("{}", res.unwrap());
    }
    #[test]
    fn test_verify_black3() {
        let key = "fixtures/black3.txt".to_string();
        let input = "fixtures/b64.txt".to_string();
        let format = TextFormat::Black3;
        let sign = "e8b707a6ea16da03010d6ae9d90276b590644d460c1f3a95af1124bde43add49".to_string();
        let res = verify(key, input, format, sign);
        assert!(res.is_ok());
    }

    #[test]
    fn test_sign_ed25519() {
        let key = "fixtures/ed25519.sk".to_string();
        let input = "fixtures/b64.txt".to_string();
        let format = TextFormat::Ed25519;
        let res = sign(key, input, format);
        println!("{}", res.unwrap());
    }

    #[test]
    fn test_verify_ed25519() {
        let key = "fixtures/ed25519.pk".to_string();
        let input = "fixtures/b64.txt".to_string();
        let format = TextFormat::Ed25519;
        let sign = "DF5108E9EB365BA8665F0AC256BE817E28C015C2BFD795B0309C0D5606CA391773F326EB143930DDA3FB8D7C3A779CCFD948BB516BC14A31F00BB96886BE1708".to_string();
        let res = verify(key, input, format, sign);
        assert!(res.is_ok());
    }

    #[test]
    fn test_generate_key_for_black3() {
        let format = TextFormat::Black3;
        let output = "fixtures".to_string();
        let res = generate_key(format, output);
        assert!(res.is_ok());
    }

    #[test]
    fn test_generate_key_for_ed25519() {
        let format = TextFormat::Ed25519;
        let output = "fixtures".to_string();
        let res = generate_key(format, output);
        assert!(res.is_ok());
    }
}
