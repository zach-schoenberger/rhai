use crate::def_package;
use crate::module::FuncReturn;
use crate::parser::INT;
use crate::result::EvalAltResult;
use crate::token::Position;

#[cfg(not(feature = "no_float"))]
use crate::parser::FLOAT;

use num_traits::{
    identities::Zero, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl,
    CheckedShr, CheckedSub,
};

use crate::stdlib::{
    boxed::Box,
    fmt::Display,
    format,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub},
};

// Checked add
pub(crate) fn add<T: Display + CheckedAdd>(x: T, y: T) -> FuncReturn<T> {
    x.checked_add(&y).ok_or_else(|| {
        Box::new(EvalAltResult::ErrorArithmetic(
            format!("Addition overflow: {} + {}", x, y),
            Position::none(),
        ))
    })
}
// Checked subtract
pub(crate) fn sub<T: Display + CheckedSub>(x: T, y: T) -> FuncReturn<T> {
    x.checked_sub(&y).ok_or_else(|| {
        Box::new(EvalAltResult::ErrorArithmetic(
            format!("Subtraction underflow: {} - {}", x, y),
            Position::none(),
        ))
    })
}
// Checked multiply
pub(crate) fn mul<T: Display + CheckedMul>(x: T, y: T) -> FuncReturn<T> {
    x.checked_mul(&y).ok_or_else(|| {
        Box::new(EvalAltResult::ErrorArithmetic(
            format!("Multiplication overflow: {} * {}", x, y),
            Position::none(),
        ))
    })
}
// Checked divide
pub(crate) fn div<T>(x: T, y: T) -> FuncReturn<T>
where
    T: Display + CheckedDiv + PartialEq + Zero,
{
    // Detect division by zero
    if y == T::zero() {
        return Err(Box::new(EvalAltResult::ErrorArithmetic(
            format!("Division by zero: {} / {}", x, y),
            Position::none(),
        )));
    }

    x.checked_div(&y).ok_or_else(|| {
        Box::new(EvalAltResult::ErrorArithmetic(
            format!("Division overflow: {} / {}", x, y),
            Position::none(),
        ))
    })
}
// Checked negative - e.g. -(i32::MIN) will overflow i32::MAX
pub(crate) fn neg<T: Display + CheckedNeg>(x: T) -> FuncReturn<T> {
    x.checked_neg().ok_or_else(|| {
        Box::new(EvalAltResult::ErrorArithmetic(
            format!("Negation overflow: -{}", x),
            Position::none(),
        ))
    })
}
// Checked absolute
pub(crate) fn abs<T: Display + CheckedNeg + PartialOrd + Zero>(x: T) -> FuncReturn<T> {
    // FIX - We don't use Signed::abs() here because, contrary to documentation, it panics
    //       when the number is ::MIN instead of returning ::MIN itself.
    if x >= <T as Zero>::zero() {
        Ok(x)
    } else {
        x.checked_neg().ok_or_else(|| {
            Box::new(EvalAltResult::ErrorArithmetic(
                format!("Negation overflow: -{}", x),
                Position::none(),
            ))
        })
    }
}
// Unchecked add - may panic on overflow
fn add_u<T: Add>(x: T, y: T) -> FuncReturn<<T as Add>::Output> {
    Ok(x + y)
}
// Unchecked subtract - may panic on underflow
fn sub_u<T: Sub>(x: T, y: T) -> FuncReturn<<T as Sub>::Output> {
    Ok(x - y)
}
// Unchecked multiply - may panic on overflow
fn mul_u<T: Mul>(x: T, y: T) -> FuncReturn<<T as Mul>::Output> {
    Ok(x * y)
}
// Unchecked divide - may panic when dividing by zero
fn div_u<T: Div>(x: T, y: T) -> FuncReturn<<T as Div>::Output> {
    Ok(x / y)
}
// Unchecked negative - may panic on overflow
fn neg_u<T: Neg>(x: T) -> FuncReturn<<T as Neg>::Output> {
    Ok(-x)
}
// Unchecked absolute - may panic on overflow
fn abs_u<T>(x: T) -> FuncReturn<<T as Neg>::Output>
where
    T: Neg + PartialOrd + Default + Into<<T as Neg>::Output>,
{
    // Numbers should default to zero
    if x < Default::default() {
        Ok(-x)
    } else {
        Ok(x.into())
    }
}
// Bit operators
fn binary_and<T: BitAnd>(x: T, y: T) -> FuncReturn<<T as BitAnd>::Output> {
    Ok(x & y)
}
fn binary_or<T: BitOr>(x: T, y: T) -> FuncReturn<<T as BitOr>::Output> {
    Ok(x | y)
}
fn binary_xor<T: BitXor>(x: T, y: T) -> FuncReturn<<T as BitXor>::Output> {
    Ok(x ^ y)
}
// Checked left-shift
pub(crate) fn shl<T: Display + CheckedShl>(x: T, y: INT) -> FuncReturn<T> {
    // Cannot shift by a negative number of bits
    if y < 0 {
        return Err(Box::new(EvalAltResult::ErrorArithmetic(
            format!("Left-shift by a negative number: {} << {}", x, y),
            Position::none(),
        )));
    }

    CheckedShl::checked_shl(&x, y as u32).ok_or_else(|| {
        Box::new(EvalAltResult::ErrorArithmetic(
            format!("Left-shift by too many bits: {} << {}", x, y),
            Position::none(),
        ))
    })
}
// Checked right-shift
pub(crate) fn shr<T: Display + CheckedShr>(x: T, y: INT) -> FuncReturn<T> {
    // Cannot shift by a negative number of bits
    if y < 0 {
        return Err(Box::new(EvalAltResult::ErrorArithmetic(
            format!("Right-shift by a negative number: {} >> {}", x, y),
            Position::none(),
        )));
    }

    CheckedShr::checked_shr(&x, y as u32).ok_or_else(|| {
        Box::new(EvalAltResult::ErrorArithmetic(
            format!("Right-shift by too many bits: {} % {}", x, y),
            Position::none(),
        ))
    })
}
// Unchecked left-shift - may panic if shifting by a negative number of bits
pub(crate) fn shl_u<T: Shl<T>>(x: T, y: T) -> FuncReturn<<T as Shl<T>>::Output> {
    Ok(x.shl(y))
}
// Unchecked right-shift - may panic if shifting by a negative number of bits
pub(crate) fn shr_u<T: Shr<T>>(x: T, y: T) -> FuncReturn<<T as Shr<T>>::Output> {
    Ok(x.shr(y))
}
// Checked modulo
pub(crate) fn modulo<T: Display + CheckedRem>(x: T, y: T) -> FuncReturn<T> {
    x.checked_rem(&y).ok_or_else(|| {
        Box::new(EvalAltResult::ErrorArithmetic(
            format!("Modulo division by zero or overflow: {} % {}", x, y),
            Position::none(),
        ))
    })
}
// Unchecked modulo - may panic if dividing by zero
fn modulo_u<T: Rem>(x: T, y: T) -> FuncReturn<<T as Rem>::Output> {
    Ok(x % y)
}
// Checked power
pub(crate) fn pow_i_i(x: INT, y: INT) -> FuncReturn<INT> {
    #[cfg(not(feature = "only_i32"))]
    {
        if y > (u32::MAX as INT) {
            Err(Box::new(EvalAltResult::ErrorArithmetic(
                format!("Integer raised to too large an index: {} ~ {}", x, y),
                Position::none(),
            )))
        } else if y < 0 {
            Err(Box::new(EvalAltResult::ErrorArithmetic(
                format!("Integer raised to a negative index: {} ~ {}", x, y),
                Position::none(),
            )))
        } else {
            x.checked_pow(y as u32).ok_or_else(|| {
                Box::new(EvalAltResult::ErrorArithmetic(
                    format!("Power overflow: {} ~ {}", x, y),
                    Position::none(),
                ))
            })
        }
    }

    #[cfg(feature = "only_i32")]
    {
        if y < 0 {
            Err(Box::new(EvalAltResult::ErrorArithmetic(
                format!("Integer raised to a negative index: {} ~ {}", x, y),
                Position::none(),
            )))
        } else {
            x.checked_pow(y as u32).ok_or_else(|| {
                Box::new(EvalAltResult::ErrorArithmetic(
                    format!("Power overflow: {} ~ {}", x, y),
                    Position::none(),
                ))
            })
        }
    }
}
// Unchecked integer power - may panic on overflow or if the power index is too high (> u32::MAX)
pub(crate) fn pow_i_i_u(x: INT, y: INT) -> FuncReturn<INT> {
    Ok(x.pow(y as u32))
}
// Floating-point power - always well-defined
#[cfg(not(feature = "no_float"))]
pub(crate) fn pow_f_f(x: FLOAT, y: FLOAT) -> FuncReturn<FLOAT> {
    Ok(x.powf(y))
}
// Checked power
#[cfg(not(feature = "no_float"))]
pub(crate) fn pow_f_i(x: FLOAT, y: INT) -> FuncReturn<FLOAT> {
    // Raise to power that is larger than an i32
    if y > (i32::MAX as INT) {
        return Err(Box::new(EvalAltResult::ErrorArithmetic(
            format!("Number raised to too large an index: {} ~ {}", x, y),
            Position::none(),
        )));
    }

    Ok(x.powi(y as i32))
}
// Unchecked power - may be incorrect if the power index is too high (> i32::MAX)
#[cfg(feature = "unchecked")]
#[cfg(not(feature = "no_float"))]
pub(crate) fn pow_f_i_u(x: FLOAT, y: INT) -> FuncReturn<FLOAT> {
    Ok(x.powi(y as i32))
}

macro_rules! reg_unary {
    ($lib:expr, $op:expr, $func:ident, $($par:ty),*) => {
        $( $lib.set_fn_1($op, $func::<$par>); )*
    };
}
macro_rules! reg_op {
    ($lib:expr, $op:expr, $func:ident, $($par:ty),*) => {
        $( $lib.set_fn_2($op, $func::<$par>); )*
    };
}
macro_rules! reg_sign {
    ($lib:expr, $op:expr, $ret:ty, $($par:ty),*) => {
        $( $lib.set_fn_1($op, |value: $par| -> Result<$ret, _> {
            Ok(if value == (0 as $par) {
                (0 as $ret)
            } else if value < (0 as $par) {
                (-1 as $ret)
            } else {
                (1 as $ret)
            })
        }); )*
    };
}

def_package!(crate:ArithmeticPackage:"Basic arithmetic", lib, {
    #[cfg(not(feature = "only_i32"))]
    #[cfg(not(feature = "only_i64"))]
    {
        #[cfg(not(feature = "unchecked"))]
        {
            // Checked basic arithmetic
            reg_op!(lib, "+", add, i8, u8, i16, u16, i32, u32, u64);
            reg_op!(lib, "-", sub, i8, u8, i16, u16, i32, u32, u64);
            reg_op!(lib, "*", mul, i8, u8, i16, u16, i32, u32, u64);
            reg_op!(lib, "/", div, i8, u8, i16, u16, i32, u32, u64);
            // Checked bit shifts
            reg_op!(lib, "<<", shl, i8, u8, i16, u16, i32, u32, u64);
            reg_op!(lib, ">>", shr, i8, u8, i16, u16, i32, u32, u64);
            reg_op!(lib, "%", modulo, i8, u8, i16, u16, i32, u32, u64);

            #[cfg(not(target_arch = "wasm32"))]
            {
                reg_op!(lib, "+", add, i128, u128);
                reg_op!(lib, "-", sub, i128, u128);
                reg_op!(lib, "*", mul, i128, u128);
                reg_op!(lib, "/", div, i128, u128);
                // Checked bit shifts
                reg_op!(lib, "<<", shl, i128, u128);
                reg_op!(lib, ">>", shr, i128, u128);
                reg_op!(lib, "%", modulo, i128, u128);
            }
        }

        #[cfg(feature = "unchecked")]
        {
            // Unchecked basic arithmetic
            reg_op!(lib, "+", add_u, i8, u8, i16, u16, i32, u32, u64);
            reg_op!(lib, "-", sub_u, i8, u8, i16, u16, i32, u32, u64);
            reg_op!(lib, "*", mul_u, i8, u8, i16, u16, i32, u32, u64);
            reg_op!(lib, "/", div_u, i8, u8, i16, u16, i32, u32, u64);
            // Unchecked bit shifts
            reg_op!(lib, "<<", shl_u, i64, i8, u8, i16, u16, i32, u32, u64);
            reg_op!(lib, ">>", shr_u, i64, i8, u8, i16, u16, i32, u32, u64);
            reg_op!(lib, "%", modulo_u, i8, u8, i16, u16, i32, u32, u64);

            #[cfg(not(target_arch = "wasm32"))]
            {
                reg_op!(lib, "+", add_u, i128, u128);
                reg_op!(lib, "-", sub_u, i128, u128);
                reg_op!(lib, "*", mul_u, i128, u128);
                reg_op!(lib, "/", div_u, i128, u128);
                // Unchecked bit shifts
                reg_op!(lib, "<<", shl_u, i128, u128);
                reg_op!(lib, ">>", shr_u, i128, u128);
                reg_op!(lib, "%", modulo_u, i128, u128);
            }
        }

        reg_sign!(lib, "sign", INT, i8, i16, i32, i64);

        #[cfg(not(target_arch = "wasm32"))]
        reg_sign!(lib, "sign", INT, i128);
    }

    // Basic arithmetic for floating-point - no need to check
    #[cfg(not(feature = "no_float"))]
    {
        reg_op!(lib, "+", add_u, f32);
        reg_op!(lib, "-", sub_u, f32);
        reg_op!(lib, "*", mul_u, f32);
        reg_op!(lib, "/", div_u, f32);
        reg_sign!(lib, "sign", f32, f32);
        reg_sign!(lib, "sign", f64, f64);
    }

    #[cfg(not(feature = "only_i32"))]
    #[cfg(not(feature = "only_i64"))]
    {
        reg_op!(lib, "|", binary_or, i8, u8, i16, u16, i32, u32, u64);
        reg_op!(lib, "&", binary_and, i8, u8, i16, u16, i32, u32, u64);
        reg_op!(lib, "^", binary_xor, i8, u8, i16, u16, i32, u32, u64);

        #[cfg(not(target_arch = "wasm32"))]
        {
            reg_op!(lib, "|", binary_or, i128, u128);
            reg_op!(lib, "&", binary_and, i128, u128);
            reg_op!(lib, "^", binary_xor, i128, u128);
        }
    }

    #[cfg(not(feature = "no_float"))]
    {
        // Checked power
        #[cfg(not(feature = "unchecked"))]
        lib.set_fn_2("~", pow_f_i);

        // Unchecked power
        #[cfg(feature = "unchecked")]
        lib.set_fn_2("~", pow_f_i_u);

        // Floating-point modulo and power
        reg_op!(lib, "%", modulo_u, f32);

        // Floating-point unary
        reg_unary!(lib, "-", neg_u, f32, f64);
        reg_unary!(lib, "abs", abs_u, f32, f64);
    }

    // Checked unary
    #[cfg(not(feature = "unchecked"))]
    {
        reg_unary!(lib, "-", neg, INT);
        reg_unary!(lib, "abs", abs, INT);

        #[cfg(not(feature = "only_i32"))]
        #[cfg(not(feature = "only_i64"))]
        {
            reg_unary!(lib, "-", neg, i8, i16, i32, i64);
            reg_unary!(lib, "abs", abs, i8, i16, i32, i64);

            #[cfg(not(target_arch = "wasm32"))]
            {
                reg_unary!(lib, "-", neg, i128);
                reg_unary!(lib, "abs", abs, i128);
            }
        }
    }

    // Unchecked unary
    #[cfg(feature = "unchecked")]
    {
        reg_unary!(lib, "-", neg_u, INT);
        reg_unary!(lib, "abs", abs_u, INT);

        #[cfg(not(feature = "only_i32"))]
        #[cfg(not(feature = "only_i64"))]
        {
            reg_unary!(lib, "-", neg_u, i8, i16, i32, i64);
            reg_unary!(lib, "abs", abs_u, i8, i16, i32, i64);

            #[cfg(not(target_arch = "wasm32"))]
            {
                reg_unary!(lib, "-", neg_u, i128);
                reg_unary!(lib, "abs", abs_u, i128);
            }
        }
    }
});
