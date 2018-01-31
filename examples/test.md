```rust
let mut scales = Scales::new();

        scales
            .with_base(1024)
            .with_suffixes(["".to_owned(),"ki".to_owned(), "Mi".to_owned(), "Gi".to_owned(), "Ti".to_owned(), "Pi".to_owned(), "Ei".to_owned(), "Zi".to_owned(), "Yi".to_owned()].to_vec());

        assert_eq!(Formatter::new()
            .with_scales(scales)
            .with_units("B")
            .format(1024 as f64),
"1.00 kiB");
```
