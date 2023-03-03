# num2english

This Rust crate provides the `NumberToEnglish` trait which can be used to convert any* number to its string representation in English.

It uses the [Conway-Wechsler system](http://www.mrob.com/pub/math/largenum.html#conway-wechsler) for converting numbers to words.
Other systems might be added in the future.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
num2english = { git = "https://github.com/trvswgnr/num2english", branch = "main" }
```

Then add this to your crate root:

```rust
use num2english::NumberToEnglish;
```

## Examples

```rust
use num2english::NumberToEnglish;

assert_eq!(6.000_052.to_english(), "six and fifty-two millionths");
assert_eq!(52.000_001.to_english(), "fifty-two and one millionth");
```

This will work even for incredibly large numbers:
```rust
assert_eq!(
    f64::MAX.to_english(),
    "one hundred seventy-nine uncentillion seven hundred sixty-nine centillion three hundred thirteen novenonagintillion four hundred eighty-six octononagintillion two hundred thirty-one septenonagintillion five hundred seventy senonagintillion"
);
```

## *Limitations

Numbers that use scientific notation when represented as a string are not supported. This means that [`BigFloat`](https://docs.rs/num-bigint/latest/num_bigint/struct.BigFloat.html) is not supported yet, but [`BigInt`](https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html) is.

## License

This project is licensed under the MIT license. See [LICENSE](LICENSE) for more details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
