//! IEEE 754 Floating Point Decoder Module

use bit_operations::BitArray;
use leptos::prelude::*;

/// IEEE 754 Format Enum
#[derive(Debug, Clone, Copy)]
pub(crate) enum IEEEFormat {
    Half,
    Single,
    Double,
}

/// IEEE 754 Decoder structure
#[derive(Debug, Clone)]
pub struct IEEEDecoder {
    pub sign: u8,
    pub exponent: i32,
    pub exponent_bits: i32,
    pub mantissa: u64,
    pub value: f64,
    pub special: String,
    pub format: IEEEFormat,
}

impl IEEEDecoder {
    pub(crate) fn new(bits: u64, bit_size: u64) -> Self {
        match bit_size {
            16 => Self::decode_half(bits as u16),
            32 => Self::decode_single(bits as u32),
            _ => Self::decode_double(bits),
        }
    }

    fn decode_half(bits: u16) -> Self {
        let sign = ((bits >> 15) & 1) as u8;
        let exponent_bits = ((bits >> 10) & 0x1F) as i32;
        let exponent = exponent_bits - 15;
        let mantissa = (bits & 0x03FF) as u64;

        let (special, value) = match (exponent_bits, mantissa) {
            (0x1F, 0) => (
                if sign == 0 { "+Inf" } else { "-Inf" },
                if sign == 0 {
                    f64::INFINITY
                } else {
                    f64::NEG_INFINITY
                },
            ),
            (0x1F, _) => ("NaN", f64::NAN),
            (0, 0) => ("Zero", 0.0),
            (0, _) => ("Denormalized", Self::half_to_f64(sign, -14, mantissa)),
            _ => ("Normalized", Self::half_to_f64(sign, exponent, mantissa)),
        };

        Self {
            sign,
            exponent,
            exponent_bits,
            mantissa,
            value,
            special: special.to_string(),
            format: IEEEFormat::Half,
        }
    }

    fn decode_single(bits: u32) -> Self {
        let sign = ((bits >> 31) & 1) as u8;
        let exponent_bits = ((bits >> 23) & 0xFF) as i32;
        let exponent = exponent_bits - 127;
        let mantissa = (bits & 0x007F_FFFF) as u64;

        let (special, value) = match (exponent_bits, mantissa) {
            (0xFF, 0) => (
                if sign == 0 { "+Inf" } else { "-Inf" },
                if sign == 0 {
                    f64::INFINITY
                } else {
                    f64::NEG_INFINITY
                },
            ),
            (0xFF, _) => ("NaN", f64::NAN),
            (0, 0) => ("Zero", 0.0),
            (0, _) => ("Denormalized", (mantissa as f64) * 2.0_f64.powi(-126)),
            _ => ("Normalized", f32::from_bits(bits) as f64),
        };

        Self {
            sign,
            exponent,
            exponent_bits,
            mantissa,
            value,
            special: special.to_string(),
            format: IEEEFormat::Single,
        }
    }

    fn decode_double(bits: u64) -> Self {
        let sign = ((bits >> 63) & 1) as u8;
        let exponent_bits = ((bits >> 52) & 0x7FF) as i32;
        let exponent = exponent_bits - 1023;
        let mantissa = bits & 0x000F_FFFF_FFFF_FFFF;

        let (special, value) = match (exponent_bits, mantissa) {
            (0x7FF, 0) => (
                if sign == 0 { "+Inf" } else { "-Inf" },
                if sign == 0 {
                    f64::INFINITY
                } else {
                    f64::NEG_INFINITY
                },
            ),
            (0x7FF, _) => ("NaN", f64::NAN),
            (0, 0) => ("Zero", 0.0),
            (0, _) => ("Denormalized", (mantissa as f64) * 2.0_f64.powi(-1022)),
            _ => ("Normalized", f64::from_bits(bits)),
        };

        Self {
            sign,
            exponent,
            exponent_bits,
            mantissa,
            value,
            special: special.to_string(),
            format: IEEEFormat::Double,
        }
    }

    fn half_to_f64(sign: u8, exponent: i32, mantissa: u64) -> f64 {
        let sign_mult = if sign == 1 { -1.0 } else { 1.0 };

        if exponent == 0 {
            // Денормализованные числа: exp = -14, без скрытой 1 в мантиссе
            sign_mult * (mantissa as f64) * 2.0f64.powi(-24) // 2^(-14 - 10)
        } else {
            // Обычные числа: exp - 15, добавляем скрытую 1 в мантиссу
            sign_mult * (1.0 + (mantissa as f64) / 1024.0) * 2.0f64.powi(exponent - 15)
        }
    }

    fn exponent_bits_count(&self) -> usize {
        match self.format {
            IEEEFormat::Half => 5,
            IEEEFormat::Single => 8,
            IEEEFormat::Double => 11,
        }
    }

    fn mantissa_bits(&self) -> usize {
        match self.format {
            IEEEFormat::Half => 10,
            IEEEFormat::Single => 23,
            IEEEFormat::Double => 52,
        }
    }
}

/// IEEE 754 Decoder Component
#[component]
pub fn IEEE754Display(bit_array: ReadSignal<BitArray>, bit_size: ReadSignal<u64>) -> impl IntoView {
    let decoder = move || IEEEDecoder::new(bit_array.get().0, bit_size.get());

    view! {
        <div class="ieee-fields">
            <div>Format: {move || format!("{:?}", decoder().format)},
                Sign: {move || decoder().sign},
                Exponent: {move || format!(
                    "0b{:0width$b} ({})",
                    decoder().exponent_bits,
                    decoder().exponent,
                    width = decoder().exponent_bits_count()
                )},
                Mantissa: {move || format!(
                    "0x{:01$x}",
                    decoder().mantissa,
                    (decoder().mantissa_bits() + 3) / 4
                )},
                Type: {move || decoder().special.clone()}
                <div>
                    Value: {move || format!("{:e}", decoder().value)}
                </div>
            </div>
        </div>
    }
}
