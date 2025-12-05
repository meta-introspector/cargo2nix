// crates/monster_traits/src/number_properties.rs

/// Checks if `value` is divisible by `factor`.
pub fn has_factor(value: u32, factor: u32) -> bool {
    if factor == 0 {
        return false; // Division by zero is undefined
    }
    value % factor == 0
}

/// If `value = base^exponent`, returns `exponent`. Otherwise, returns `None`.
pub fn get_exponent_property(value: u32, base: u32) -> Option<u32> {
    if base == 0 {
        return if value == 0 { Some(1) } else { None }; // 0^0 is often 1, 0^positive is 0. Handle as appropriate.
    }
    if base == 1 {
        return if value == 1 { Some(0) } else { None }; // 1^x is always 1
    }

    let mut current_value = value;
    let mut exponent = 0;

    while current_value > 1 {
        if current_value % base != 0 {
            return None; // Not a perfect power of base
        }
        current_value /= base;
        exponent += 1;
    }

    if current_value == 1 { Some(exponent) } else { None } // If value was 1, exponent is 0
}

/// Returns X if `value = N * X`. Otherwise, returns `None`.
pub fn divides_into_x_parts(value: u32, n: u32) -> Option<u32> {
    if n == 0 {
        return None; // Division by zero is undefined
    }
    if value % n == 0 {
        Some(value / n)
    } else {
        None
    }
}

/// Returns the number of "parts" for a primitive type.
/// For u32, it's typically considered 1 part (itself).
pub fn get_type_parts_count_for_primitive<T>() -> u32 {
    1
}

/// Returns the number of variants in an enum.
pub fn get_type_parts_count_for_enum_variants(num_variants: u32) -> u32 {
    num_variants
}

