//! Special Value Generator Module

use bit_operations::BitArray;
use leptos::prelude::*;

const SPECIAL_VALUES: &[(&str, fn(u64) -> u64)] = &[
    ("NaN (Quiet)", |bits| match bits {
        16 => 0x7E00,
        32 => 0x7FC00000,
        64 => 0x7FF8000000000000,
        _ => 0,
    }),
    ("NaN (Signaling)", |bits| match bits {
        16 => 0x7C01,
        32 => 0x7F800001,
        64 => 0x7FF0000000000001,
        _ => 0,
    }),
    ("+Inf", |bits| match bits {
        16 => 0x7C00,
        32 => 0x7F800000,
        64 => 0x7FF0000000000000,
        _ => 0,
    }),
    ("-Inf", |bits| match bits {
        16 => 0xFC00,
        32 => 0xFF800000,
        64 => 0xFFF0000000000000,
        _ => 0,
    }),
    ("+0", |_| 0),
    ("-0", |bits| match bits {
        16 => 0x8000,
        32 => 0x80000000,
        64 => 0x8000000000000000,
        _ => 0,
    }),
    ("Min Pos", |bits| match bits {
        16 => 0x0001,
        32 => 0x00000001,
        64 => 0x0000000000000001,
        _ => 0,
    }),
    ("Max Pos", |bits| match bits {
        16 => 0x7BFF,
        32 => 0x7F7FFFFF,
        64 => 0x7FEFFFFFFFFFFFFF,
        _ => 0,
    }),
];

/// Special Value Generator Component
#[component]
pub fn SpecialValueGenerator(
    set_bit_array: WriteSignal<BitArray>,
    bit_size: ReadSignal<u64>,
) -> impl IntoView {
    let mask = move || match bit_size.get() {
        16 => 0xFFFFu64,
        32 => 0xFFFFFFFFu64,
        _ => u64::MAX,
    };

    view! {
            <label>
                <span class="input-label">Special Value Generator</span>
            </label>
            <div class="bit-operations">
                {SPECIAL_VALUES.iter().map(|(name, gen)| {
                    let value = move || gen(bit_size.get()) & mask();
                    view! {
                        <button
                            class="bit-btn"
                            on:click=move |_| set_bit_array.set(BitArray(value()))
                            disabled=move || value() == 0
                        >
                            {*name}
                        </button>
                    }
                }).collect_view()}
            </div>
    }
}
