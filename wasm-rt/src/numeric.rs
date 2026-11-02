#[cold]
#[track_caller]
pub fn trap(what: &str) -> ! {
    panic!("wasm trap: {what}")
}

pub fn wasm_unreachable() -> ! {
    trap("unreachable")
}

macro_rules! integer_ops {
    ($s:ty, $u:ty, $ds:ident, $du:ident, $rs:ident, $ru:ident) => {
        #[inline]
        pub fn $ds(a: $s, b: $s) -> $s {
            a.checked_div(b)
                .unwrap_or_else(|| trap("integer divide by zero or overflow"))
        }
        #[inline]
        pub fn $du(a: $s, b: $s) -> $s {
            (a as $u)
                .checked_div(b as $u)
                .unwrap_or_else(|| trap("integer divide by zero")) as $s
        }
        #[inline]
        pub fn $rs(a: $s, b: $s) -> $s {
            if b == 0 {
                trap("integer remainder by zero")
            }
            a.wrapping_rem(b)
        }
        #[inline]
        pub fn $ru(a: $s, b: $s) -> $s {
            (a as $u)
                .checked_rem(b as $u)
                .unwrap_or_else(|| trap("integer remainder by zero")) as $s
        }
    };
}

integer_ops!(i32, u32, i32_div_s, i32_div_u, i32_rem_s, i32_rem_u);
integer_ops!(i64, u64, i64_div_s, i64_div_u, i64_rem_s, i64_rem_u);

macro_rules! truncation {
    ($name:ident, $sat:ident, $f:ty, $s:ty, $cast:ty, $lo:expr, $hi:expr) => {
        #[inline]
        pub fn $name(value: $f) -> $s {
            let v = value.trunc();
            if !(v >= $lo && v < $hi) {
                trap("invalid conversion to integer")
            }
            (v as $cast) as $s
        }
        #[inline]
        pub fn $sat(value: $f) -> $s {
            (value as $cast) as $s
        }
    };
}

truncation!(
    i32_trunc_f32_s,
    i32_trunc_sat_f32_s,
    f32,
    i32,
    i32,
    -2147483648.0,
    2147483648.0
);
truncation!(
    i32_trunc_f32_u,
    i32_trunc_sat_f32_u,
    f32,
    i32,
    u32,
    0.0,
    4294967296.0
);
truncation!(
    i32_trunc_f64_s,
    i32_trunc_sat_f64_s,
    f64,
    i32,
    i32,
    -2147483648.0,
    2147483648.0
);
truncation!(
    i32_trunc_f64_u,
    i32_trunc_sat_f64_u,
    f64,
    i32,
    u32,
    0.0,
    4294967296.0
);
truncation!(
    i64_trunc_f32_s,
    i64_trunc_sat_f32_s,
    f32,
    i64,
    i64,
    -9223372036854775808.0,
    9223372036854775808.0
);
truncation!(
    i64_trunc_f32_u,
    i64_trunc_sat_f32_u,
    f32,
    i64,
    u64,
    0.0,
    18446744073709551616.0
);
truncation!(
    i64_trunc_f64_s,
    i64_trunc_sat_f64_s,
    f64,
    i64,
    i64,
    -9223372036854775808.0,
    9223372036854775808.0
);
truncation!(
    i64_trunc_f64_u,
    i64_trunc_sat_f64_u,
    f64,
    i64,
    u64,
    0.0,
    18446744073709551616.0
);

macro_rules! select {
    ($name:ident, $ty:ty) => {
        #[inline]
        pub fn $name(condition: i32, on_false: $ty, on_true: $ty) -> $ty {
            if condition != 0 {
                on_true
            } else {
                on_false
            }
        }
    };
}
select!(select_i32, i32);
select!(select_i64, i64);
select!(select_f32, f32);
select!(select_f64, f64);
select!(select_v128, u128);
