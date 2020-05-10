# `Iuliia`
> Transliterate Cyrillic → Latin in every possible way

> This is the port of the incredible python library [iuliia](https://github.com/nalgeon/iuliia-py) made by @nalgeon

Transliteration means representing Cyrillic data (mainly names and geographic locations) with Latin letters. It is used for international passports, visas, green cards, driving licenses, mail and goods delivery etc.

`Iuliia` makes transliteration as easy as:

```rust
fn main() {
    let schema = iuliia::schemas::IcaoDoc9303::new();
    let o = schema.transliterate("Юлия Щеглова".to_string());
    println!("{0}", o);
}

>> 'Iuliia Shcheglova'
```


## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Make sure to add or update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)