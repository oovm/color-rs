use super::*;

impl Parse for RgbaColor {
    fn parse(input: ParseStream) -> Result<Self> {
        rgba_from_string(input).or_else(|_| rgba_from_number(input))
    }
}

pub fn rgba_from_string(input: ParseStream) -> Result<RgbaColor> {
    let span = input.span();
    if input.peek(LitStr) {
        let string: LitStr = input.parse()?;
        let string = string.value();
        let string = string.trim();
        if string.starts_with('#') {
            return parse_hex(string, span);
        }
        if string.starts_with("rgb") {
            return parse_rgba(string, span);
        }
    }
    Err(Error::new(span, "Unknown color pattern".to_string()))
}

pub fn rgba_from_number(input: ParseStream) -> Result<RgbaColor> {
    let r = next_number(input, 255.0)?;
    let _ = input.parse::<Token![,]>()?;
    let g = next_number(input, 255.0)?;
    let _ = input.parse::<Token![,]>()?;
    let b = next_number(input, 255.0)?;
    if let false = input.peek(Token![,]) {
        return Ok(RgbaColor { rgba32: RGBA32::new(r, g, b, 1.0) });
    }
    let _ = input.parse::<Token![,]>()?;
    let a = next_number(input, 255.0)?;
    Ok(RgbaColor { rgba32: RGBA32::new(r, g, b, a) })
}
