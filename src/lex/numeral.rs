use {
    super::Token,
    lexical::{parse_float_options, NumberFormatBuilder},
    logos::Lexer,
};

#[derive(Clone, Copy, Debug)]
pub enum Numeral {
    Int(i64),
    Float(u64),
}

const DEC_FLOAT_FORMAT: u128 = NumberFormatBuilder::new().no_special(true).build();

const HEX_FLOAT_FORMAT: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .mantissa_radix(16)
    .build();

pub fn dec_int_callback(lexer: &mut Lexer<Token>) -> Numeral {
    match lexical::parse(lexer.slice()) {
        Ok(v) => Numeral::Int(v),

        Err(_) => Numeral::Float(
            lexical::parse_with_options::<f64, _, DEC_FLOAT_FORMAT>(
                lexer.slice(),
                &parse_float_options::STANDARD,
            )
            .unwrap()
            .to_bits(),
        ),
    }
}

pub fn hex_int_callback(lexer: &mut Lexer<Token>) -> Numeral {
    let mut total = 0i64;

    for c in &lexer.slice()[2..] {
        total = total.wrapping_mul(16);
        total = total.wrapping_add(match c {
            b'0'..=b'9' => (c - b'0') as _,
            b'a'..=b'f' => (c - b'a' + 10) as _,
            _ => (c - b'A' + 10) as _,
        })
    }

    Numeral::Int(total)
}

pub fn dec_float_callback(lexer: &mut Lexer<Token>) -> Numeral {
    Numeral::Float(
        lexical::parse_with_options::<f64, _, DEC_FLOAT_FORMAT>(
            lexer.slice(),
            &parse_float_options::STANDARD,
        )
        .unwrap()
        .to_bits(),
    )
}

pub fn hex_float_callback(lexer: &mut Lexer<Token>) -> Numeral {
    Numeral::Float(
        lexical::parse_with_options::<f64, _, HEX_FLOAT_FORMAT>(
            &lexer.slice()[2..],
            &parse_float_options::HEX_FLOAT,
        )
        .unwrap()
        .to_bits(),
    )
}
