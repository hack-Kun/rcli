use rand::seq::{IndexedRandom, SliceRandom};
use zxcvbn::zxcvbn;

const LOWER_CASE: &[u8] = b"abcdefghjklmnopqrstuvwxyz";
const UPPER_CASE: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"@%^*_.";
pub fn generate(
    length: u8,
    lower: bool,
    upper: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::with_capacity(length as usize);
    let mut chars = Vec::new();
    // 为了确保每一种用户选择的数据都能出现在密码中，我们先在每一种字符中随机选择一个添加到密码中，
    // 然后再从剩余的字符中随机选择，直到密码长度达到要求。
    if !upper {
        password.push(*UPPER_CASE.choose(&mut rng).unwrap());
        chars.extend_from_slice(UPPER_CASE);
    }
    if !lower {
        password.push(*LOWER_CASE.choose(&mut rng).unwrap());
        chars.extend_from_slice(LOWER_CASE);
    }
    if !number {
        password.push(*NUMBER.choose(&mut rng).unwrap());
        chars.extend_from_slice(NUMBER);
    }
    if !symbol {
        password.push(*SYMBOL.choose(&mut rng).unwrap());
        chars.extend_from_slice(SYMBOL);
    }

    for _ in 0..(length - password.len() as u8) {
        password.push(*chars.choose(&mut rng).unwrap());
    }
    // shuffle the password
    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;

    println!("{password}");

    // 检测密码强度
    let passwd_stronger = zxcvbn(&password, &[]);
    eprintln!("密码强度为: {}", passwd_stronger.score());

    Ok(())
}
