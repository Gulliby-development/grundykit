/// Returns the nim product for two 32-bit nimbers.
pub fn product(a: u32, b: u32) -> u32 {
    product_with_half(a, b, 16)
}

/// Multiplies values below 2^(2h) using the recursive half-field identity.
///
/// For h > 1, splitting each operand at h bits gives five products in the
/// field below 2^h. The fixed h sequence is 16, 8, 4, 2, 1, so each product
/// uses at most 5^5 = 3,125 one-bit AND operations.
fn product_with_half(x: u32, y: u32, h: u32) -> u32 {
    if h == 1 {
        let x0 = x & 1;
        let x1 = x >> 1;
        let y0 = y & 1;
        let y1 = y >> 1;
        let high_product = x1 & y1;
        let cross = (x1 & y0) ^ (x0 & y1) ^ high_product;
        let low_product = x0 & y0;
        return (cross << 1) ^ low_product ^ high_product;
    }

    let mask = (1_u32 << h) - 1;
    let x0 = x & mask;
    let x1 = x >> h;
    let y0 = y & mask;
    let y1 = y >> h;
    let half = h / 2;

    let high_product = product_with_half(x1, y1, half);
    let cross = product_with_half(x1, y0, half) ^ product_with_half(x0, y1, half) ^ high_product;
    let low_product = product_with_half(x0, y0, half);
    let scaled_high_product = product_with_half(high_product, 1_u32 << (h - 1), half);

    (cross << h) ^ low_product ^ scaled_high_product
}

/// Returns the multiplicative inverse, or None for zero.
///
/// A nonzero 32-bit nimber has inverse a^(2^32 - 2). Square-and-multiply
/// takes at most 32 iterations and at most 63 products; each product has a
/// fixed recursion depth and at most 3,125 one-bit AND operations.
pub fn inverse(a: u32) -> Option<u32> {
    if a == 0 {
        return None;
    }

    let mut result = 1;
    let mut base = a;
    let mut exponent = 4_294_967_294_u64;

    while exponent != 0 {
        if exponent & 1 != 0 {
            result = product(result, base);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = product(base, base);
        }
    }

    Some(result)
}
