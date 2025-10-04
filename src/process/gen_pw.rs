use anyhow::Result;
use rand::prelude::*;
use zxcvbn::zxcvbn;

const UPPERCASE: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const DIGITS: &[u8] = b"23456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=<>?";

pub fn generate_password(
    length: u32,
    uppercase: bool,
    lowercase: bool,
    digits: bool,
    symbols: bool,
) -> Result<String> {
    let mut password = Vec::new();
    let mut choices = Vec::new();
    let mut rng = rand::rng();
    if uppercase {
        choices.extend_from_slice(UPPERCASE);
        password.push(
            *UPPERCASE
                .choose(&mut rng)
                .expect("UPPERCASE won't be empty"),
        );
    }
    if lowercase {
        choices.extend_from_slice(LOWERCASE);
        password.push(
            *LOWERCASE
                .choose(&mut rng)
                .expect("LOWERCASE won't be empty"),
        );
    }
    if digits {
        choices.extend_from_slice(DIGITS);
        password.push(*DIGITS.choose(&mut rng).expect("DIGITS won't be empty"));
    }
    if symbols {
        choices.extend_from_slice(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).expect("SYMBOLS won't be empty"));
    }
    let p_len = password.len() as u32;
    if length <= p_len {
        anyhow::bail!("密码长度不能小于{}", password.len());
    }

    for _ in p_len..length {
        let index = rng.random_range(0..choices.len());
        password.push(choices[index]);
    }
    password.shuffle(&mut rng);
    let password = String::from_utf8(password).unwrap();

    let estimate = zxcvbn(&password, &[]);
    println!("密码强度:{}", estimate.score());

    Ok(password)
}
