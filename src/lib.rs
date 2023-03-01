mod scales;
use num_bigint::{BigInt, Sign};
use scales::{DECIMALS, MAGNITUDES, ONE_TO_NINETEEN, TENS};

/// Convert any number type to its name in English.
pub trait ToEnglish<T: std::fmt::Display> {
    fn to_english(&self) -> String;
}

impl<T: std::fmt::Display> ToEnglish<T> for T {
    fn to_english(&self) -> String {
        convert_number_to_english(self.to_string().parse::<f64>().unwrap())
    }
}

/// Convert a number to its name in English (e.g. 60.212 -> "sixty and two hundred twelve thousandths")
fn convert_number_to_english(number: f64) -> String {
    let (before_decimal, after_decimal, decimal_places) = split_number(number);

    let mut result = String::new();

    if let Some(mut before_decimal) = before_decimal {
        // check the sign
        match before_decimal.sign() {
            Sign::Minus => {
                result.push_str("negative ");
                before_decimal = -before_decimal;
            }
            Sign::NoSign => {
                result.push_str("zero");
                return result;
            }
            Sign::Plus => (),
        }
        result.push_str(&convert_integer_to_english(before_decimal));
    }

    if let Some(after_decimal) = after_decimal {
        result.push_str(" and ");
        result.push_str(&convert_decimal_to_english(after_decimal, decimal_places));
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

/// convert the decimal part of a number to its name in English (e.g. 60.212 -> "two hundred twelve thousandths")
fn convert_decimal_to_english(number: BigInt, decimal_places: usize) -> String {
    let mut result = String::new();
    let mut number = number;

    // get the suffix from the number of digits (e.g. 1 -> "thousandth", 2 -> "hundredth", 3 -> "tenths", etc...)
    let mut suffix = DECIMALS[decimal_places - 1].to_owned();
    if number > BigInt::from(1) {
        suffix = suffix.to_owned() + "s";
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

/// Split a number into its integer and decimal parts.
fn split_number(number: f64) -> (Option<BigInt>, Option<BigInt>, usize) {
    let string = number.to_string();
    let split = string.split('.').collect::<Vec<&str>>();
    let before_decimal = split[0].parse::<BigInt>().unwrap();
    let after_decimal = if split.len() > 1 {
        Some(split[1].parse::<BigInt>().unwrap())
    } else {
        None
    };
    let mut decimal_places = 0;
    if after_decimal.is_some() {
        decimal_places = split[1].len();
    }
    (Some(before_decimal), after_decimal, decimal_places)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_names() {
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
    }
}
