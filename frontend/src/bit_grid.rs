//! Bit Grid Visualization Module

use bit_operations::BitArray;
use leptos::prelude::CustomAttribute;
use leptos::prelude::Update;
use leptos::prelude::{
    ClassAttribute, CollectView, ElementChild, Get, OnAttribute, ReadSignal, WriteSignal,
};
use leptos::*;

/// Bit Grid Component
#[component]
pub fn BitGrid(
    bit_array: ReadSignal<BitArray>,
    set_bit_array: WriteSignal<BitArray>,
    bit_size: ReadSignal<u64>,
) -> impl IntoView {
    let mask = move || match bit_size.get() {
        8 => 0xFFu64,
        16 => 0xFFFFu64,
        32 => 0xFFFFFFFFu64,
        _ => u64::MAX,
    };

    view! {
        <div class="bit-grid">
            {(0..64).rev().map(|bit_index| {
                let is_active = move || bit_index < bit_size.get();
                let current_mask = mask();
                let bit_value = move || {
                    let current_bits = bit_array.get().0 & current_mask;
                    (current_bits >> bit_index) & 1 == 1
                };

                view! {
                    <div
                        class="bit"
                        class:active=move || is_active() && bit_value()
                        class:inactive=move || !is_active()
                        on:click=move |_| {
                            if is_active() {
                                set_bit_array.update(|ba| {
                                    ba.0 ^= 1u64 << bit_index;
                                    ba.0 &= current_mask;
                                });
                            }
                        }
                        data-bit=bit_index
                    >
                        {move || if bit_value() { "1" } else { "0" }}
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
