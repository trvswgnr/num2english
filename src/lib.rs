#![no_std]
#![deny(missing_docs)]

//! A trait for converting a number to its equivalent representation in English.
//!
//! A number is split into its integer and decimal parts with via the [`SplitNumber`] struct.
//! A [`SplitNumber`] is represented as an Option<[`BigInt`]> for the integer and decimal parts, and a usize for the number of decimal places.
//!
//! Converting a number to its English representation is done via the [`NumberToEnglish`] trait.
//! The [`NumberToEnglish`] trait is implemented for all types that implement the [`Num`] trait.
//!
//! **_Scientific notation is not supported at this time._**
//!
//! # Example
//!
//! ```
//! use num2english::NumberToEnglish;
//! assert_eq!(60.to_english(), "sixty");
//! assert_eq!(60.212.to_english(), "sixty and two hundred twelve thousandths");
//! assert_eq!((-60.212).to_english(), "negative sixty and two hundred twelve thousandths");
//! ```
//!
//! [`SplitNumber`]: struct.SplitNumber.html
//! [`NumberToEnglish`]: trait.NumberToEnglish.html
//! [`Num`]: https://docs.rs/num/latest/num/trait.Num.html
//! [`BigInt`]: https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html
//!

extern crate alloc;

mod scales;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::Display;
use num_bigint::{BigInt, Sign};
use num_traits::Num;
use scales::{DECIMALS, MAGNITUDES, ONE_TO_NINETEEN, TENS};

/// Represents a number split into its integer and decimal parts.
///
/// # Examples
/// ```
/// use num2english::SplitNumber;
/// let number = SplitNumber {
///    integer: Some(60.into()),
///   decimal: Some(212.into()),
///   decimal_places: 3,
/// };
/// assert_eq!(number.integer, Some(60.into()));
/// assert_eq!(number.decimal, Some(212.into()));
/// assert_eq!(number.decimal_places, 3);
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SplitNumber {
    /// The integer part of the number.
    pub integer: Option<BigInt>,
    /// The decimal part of the number.
    pub decimal: Option<BigInt>,
    /// The number of decimal places.
    pub decimal_places: usize,
}

/// Convert any number type to its name in English.
///
/// # Examples
/// ```
/// use num2english::NumberToEnglish;
/// assert_eq!(60.to_english(), "sixty");
/// assert_eq!(60.212.to_english(), "sixty and two hundred twelve thousandths");
/// assert_eq!((-60.212).to_english(), "negative sixty and two hundred twelve thousandths");
/// ```
pub trait NumberToEnglish<T>
where
    T: Num + Display,
{
    /// Convert a number to its English representation.
    ///
    /// **_Scientific notation is not supported... yet._**
    fn to_english(&self) -> String;
}

impl<T> NumberToEnglish<T> for T
where
    T: Num + Display,
{
    fn to_english(&self) -> String {
        let string = self.to_string();
        if string.contains('e') {
            panic!("Scientific notation is not supported at this time.");
        }
        convert_number_to_english(string)
    }
}

/// Convert a number to its name in English (e.g. 60.212 -> "sixty and two hundred twelve thousandths")
fn convert_number_to_english(number: String) -> String {
    let SplitNumber {
        integer: before_decimal,
        decimal: after_decimal,
        decimal_places,
    } = split_number(&number);

    let mut result = String::new();

    let has_integer = before_decimal.is_some();

    if let Some(mut before_decimal) = before_decimal {
        // check the sign
        if let Sign::Minus = before_decimal.sign() {
            result.push_str("negative ");
            before_decimal = -before_decimal;
        }
        result.push_str(&convert_integer_to_english(before_decimal));
    }

    if let Some(after_decimal) = after_decimal {
        if has_integer {
            result.push_str(" and ");
        }
        result.push_str(&convert_decimal_to_english(after_decimal, decimal_places));
    }

    if result.is_empty() {
        result.push_str("zero");
    }

    result
}

/// Convert an integer to its name in English (e.g. 60 -> "sixty")
fn convert_integer_to_english(number: BigInt) -> String {
    let mut result = String::new();
    let mut number = number;
    let mut magnitude = 0;

    while number > BigInt::from(0) {
        let remainder = number.clone() % BigInt::from(1000);
        number = (number - remainder.clone()) / BigInt::from(1000);

        if remainder > BigInt::from(0) {
            let mut remainder_string = convert_hundreds_to_english(remainder);
            if magnitude > 0 {
                remainder_string.push(' ');
                remainder_string.push_str(MAGNITUDES[magnitude - 1]);
            }
            if !result.is_empty() {
                remainder_string.push(' ');
            }
            remainder_string.push_str(&result);
            result = remainder_string;
        }

        magnitude += 1;
    }

    result
}

/// Converts the decimal part of a number to its name in English (e.g. 60.212 -> "two hundred twelve thousandths")
fn convert_decimal_to_english(number: BigInt, decimal_places: usize) -> String {
    let mut result = String::new();
    let mut number = number;

    // get the suffix from the number of digits (e.g. 1 -> "thousandth", 2 -> "hundredth", 3 -> "tenths", etc...)
    let mut suffix = DECIMALS[decimal_places - 1].to_string();
    if number > BigInt::from(1) {
        suffix += "s";
    }

    let mut magnitude = 0;
    while number > BigInt::from(0) && magnitude < 3 {
        let remainder = number.clone() % BigInt::from(1000);
        number = (number - remainder.clone()) / BigInt::from(1000);

        if remainder > BigInt::from(0) {
            let mut remainder_string = convert_hundreds_to_english(remainder);
            if magnitude > 0 {
                remainder_string.push(' ');
                remainder_string.push_str(MAGNITUDES[magnitude]);
            }
            if !result.is_empty() {
                remainder_string.push(' ');
            }
            remainder_string.push_str(&result);
            result = remainder_string;
        }

        magnitude += 1;
    }

    result.push(' ');
    result.push_str(&suffix);

    result
}

/// Convert a number between 0 and 999 to its name.
fn convert_hundreds_to_english(number: BigInt) -> String {
    let mut result = String::new();
    let mut number = number.to_string().parse::<u64>().unwrap();

    let hundreds = number / 100;
    number %= 100;

    if hundreds > 0 {
        result.push_str(ONE_TO_NINETEEN[(hundreds - 1) as usize]);
        result.push_str(" hundred");
        if number > 0 {
            result.push(' ');
        }
    }

    if number > 0 {
        if number < 20 {
            result.push_str(ONE_TO_NINETEEN[(number - 1) as usize]);
        } else {
            let tens = number / 10;
            number %= 10;
            result.push_str(TENS[(tens - 1) as usize]);
            if number > 0 {
                result.push('-');
                result.push_str(ONE_TO_NINETEEN[(number - 1) as usize]);
            }
        }
    }

    result
}

fn parse_big_int(n: &str) -> Option<BigInt> {
    let string = n.to_string();
    let bigint = BigInt::parse_bytes(string.as_bytes(), 10).unwrap_or_else(|| BigInt::from(0));
    if bigint == BigInt::from(0) {
        None
    } else {
        Some(bigint)
    }
}

/// Split a number into its integer and decimal parts.
fn split_number(string: &str) -> SplitNumber {
    let split = string.split('.').collect::<Vec<&str>>();
    let integer = parse_big_int(split[0]);
    let decimal = if split.len() > 1 {
        parse_big_int(split[1])
    } else {
        None
    };

    let decimal_places = if split.len() > 1 { split[1].len() } else { 0 };

    SplitNumber {
        integer,
        decimal,
        decimal_places,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        for i in 1..20 {
            let name = i.to_english();
            assert_eq!(name, ONE_TO_NINETEEN[i - 1]);
        }

        let two_hundred_fifty_five: u8 = 255;
        assert_eq!(
            two_hundred_fifty_five.to_english(),
            "two hundred fifty-five"
        );
    }

    #[test]
    fn test_i64() {
        let negative_one = (-1_i64).to_english();
        assert_eq!(negative_one, "negative one");

        let negative_one_hundred = (-100_i64).to_english();
        assert_eq!(negative_one_hundred, "negative one hundred");

        let negative_one_hundred_twenty_three = (-123_i64).to_english();
        assert_eq!(
            negative_one_hundred_twenty_three,
            "negative one hundred twenty-three"
        );

        let negative_one_hundred_twenty_three_thousand_four_hundred_fifty_six =
            (-123_456_i64).to_english();
        assert_eq!(
            negative_one_hundred_twenty_three_thousand_four_hundred_fifty_six,
            "negative one hundred twenty-three thousand four hundred fifty-six"
        );
    }

    #[test]
    fn test_random() {
        let one_million = 1_000_000.to_english();
        assert_eq!(one_million, "one million");

        let one_billion = 1_000_000_000.to_english();
        assert_eq!(one_billion, "one billion");

        let one_hundred_twenty_three = 123.to_english();
        assert_eq!(one_hundred_twenty_three, "one hundred twenty-three");

        let one_hundred_twenty_three_thousand_four_hundred_fifty_six = 123_456.to_english();
        assert_eq!(
            one_hundred_twenty_three_thousand_four_hundred_fifty_six,
            "one hundred twenty-three thousand four hundred fifty-six"
        );

        let maxf64 = f64::MAX;
        let maxf64_name = maxf64.to_english();
        assert_eq!(maxf64_name, "one hundred seventy-nine uncentillion seven hundred sixty-nine centillion three hundred thirteen novenonagintillion four hundred eighty-six octononagintillion two hundred thirty-one septenonagintillion five hundred seventy senonagintillion");

        let negative_one_hundred_twenty_three = (-123).to_english();
        assert_eq!(
            negative_one_hundred_twenty_three,
            "negative one hundred twenty-three"
        );

        let six_and_two_tenths = 6.2.to_english();
        assert_eq!(six_and_two_tenths, "six and two tenths");

        let six_hundred_thousand_and_twenty_one_hundredths = 600_000.21.to_english();
        assert_eq!(
            six_hundred_thousand_and_twenty_one_hundredths,
            "six hundred thousand and twenty-one hundredths"
        );

        let five_and_twenty_three_thousandths = 5.023.to_english();
        assert_eq!(
            five_and_twenty_three_thousandths,
            "five and twenty-three thousandths"
        );

        let six = 6.to_english();
        assert_eq!(six, "six");

        let six_and_fifty_two_millionths = 6.000_052.to_english();
        assert_eq!(six_and_fifty_two_millionths, "six and fifty-two millionths");

        let fifty_two_and_one_millionth = 52.000_001.to_english();
        assert_eq!(fifty_two_and_one_millionth, "fifty-two and one millionth");

        let zero = 0.0.to_english();
        assert_eq!(zero, "zero");

        let negative_zero = (-0.0).to_english();
        assert_eq!(negative_zero, "zero");

        let negative_zero_point_zero = (-0.0).to_english();
        assert_eq!(negative_zero_point_zero, "zero");

        let fifty_six_thousandths = 0.056.to_english();
        assert_eq!(fifty_six_thousandths, "fifty-six thousandths");
    }

    #[test]
    fn test_bigint() {
        let bigint_num = BigInt::parse_bytes(b"1234", 10).unwrap();
        let bigint_num_name = bigint_num.to_english();
        assert_eq!(bigint_num_name, "one thousand two hundred thirty-four");
    }

    #[test]
    #[should_panic]
    fn test_big_float_panic() {
        use num_bigfloat::BigFloat;
        let bigfloat_num = BigFloat::from(1234.5678);
        let bigfloat_num_name = bigfloat_num.to_english();
        assert_eq!(bigfloat_num_name, "one thousand two hundred thirty-four and five thousand six hundred seventy-eight hundredths");
    }
}
