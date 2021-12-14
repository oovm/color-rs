Rust Color
==========

### Compile time color parsing!


```rust
use color_core::{RGBA};
use color_macro::{rgba};

#[test]
fn test_rgba() {
    assert_eq!(rgba!("#334D6677"), RGBA::new(51, 76, 102));
    assert_eq!(rgba!("rgba(51, 77, 102, .5)"), RGBA::new(51, 77, 102, 127));
    assert_eq!(rgba!("rgba(20% 30% 40% 50%)"), RGBA::new(51, 77, 102, 127));
}
```

![image](https://user-images.githubusercontent.com/17541209/156918397-cf9024dc-7f2e-4f36-b3a7-2eec1cb26584.png)


### Compile time error report

```rust
#[test]
fn panic_hex5() {
    rgba!("#34678");
}
```


![image](https://user-images.githubusercontent.com/17541209/156918188-e8d6ed8c-b811-4f20-9159-040eeab5af07.png)
