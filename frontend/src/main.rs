use leptos::prelude::*;
use bit_operations::BitArray;

/// The main application component. It provides a UI to manipulate and visualize a 64-bit array.
/// The bit array can be modified through various input fields (DEC, BIN, HEX, OCT) and bitwise operations.
#[component]
fn App() -> impl IntoView {
    // Signal to track the current bit array and allow updates.
    let (bit_array, set_bit_array) = signal(BitArray::new());
    // Signal to control whether the bit visualization grid is shown.
    let (show_bits, _set_show_bits) = signal(true);

    // Signals for input fields.
    let (dec_input, set_dec_input) = signal(String::new());
    let (bin_input, set_bin_input) = signal(String::new());
    let (hex_input, set_hex_input) = signal(String::new());
    let (oct_input, set_oct_input) = signal(String::new());

    // Effect to synchronize input fields with the current bit array.
    Effect::new(move |_| {
        let current = bit_array.get().0;
        set_dec_input.set(current.to_string());
        set_bin_input.set(format!("0b{:b}", current));
        set_hex_input.set(format!("0x{:X}", current));
        set_oct_input.set(format!("0o{:o}", current));
    });

    // Generic function to update the bit array value.
    let update_value = move |value: u64| {
        set_bit_array.set(BitArray(value));
    };

    // Handlers for input field changes.
    let input_dec = move |ev: web_sys::Event| {
        let val = event_target_value(&ev);
        set_dec_input.set(val.clone());
        if let Ok(num) = val.parse::<u64>() {
            update_value(num);
        }
    };

    let input_bin = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev);
        val = val.replace("0b", "").replace(' ', "");
        set_bin_input.set(format!("0b{}", val));
        if let Ok(num) = u64::from_str_radix(&val, 2) {
            update_value(num);
        }
    };

    let input_hex = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev);
        val = val.replace("0x", "").replace(' ', "");
        set_hex_input.set(format!("0x{}", val));
        if let Ok(num) = u64::from_str_radix(&val, 16) {
            update_value(num);
        }
    };

    let input_oct = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev);
        val = val.replace("0o", "").replace(' ', "");
        set_oct_input.set(format!("0o{}", val));
        if let Ok(num) = u64::from_str_radix(&val, 8) {
            update_value(num);
        }
    };

    // Handlers for bitwise operations.
    let lsh = move |_| set_bit_array.update(|ba| *ba = BitArray(ba.0 << 1));
    let rsh = move |_| set_bit_array.update(|ba| *ba = BitArray(ba.0 >> 1));
    let not = move |_| set_bit_array.update(|ba| *ba = BitArray(!ba.0));
    let clear = move |_| set_bit_array.set(BitArray(0));
    let set_all = move |_| set_bit_array.set(BitArray(u64::MAX));

    // Handlers for circular shifts.
    let lshr = move |_| {
        set_bit_array.update(|ba| {
            let new_val = (ba.0 << 1) | (ba.0 >> 63);
            *ba = BitArray(new_val);
        });
    };

    let rshr = move |_| {
        set_bit_array.update(|ba| {
            let new_val = (ba.0 >> 1) | (ba.0 << 63);
            *ba = BitArray(new_val);
        });
    };

    view! {
        // Conditional rendering for the bit visualization grid.
        <Show when=move || show_bits.get()>
            <div class="main-container">
                {/* Bit visualization grid */}
                <div class="bit-grid">
                    {(0..64).map(|i| {
                        let bit_index = 63 - i;
                        view! {
                            <div
                                class="bit"
                                class:active=move || bit_array.get().get_bit(bit_index)
                                on:click=move |_| set_bit_array.update(|ba| ba.toggle_bit(bit_index))
                                data-bit=bit_index
                            >
                                {move || if bit_array.get().get_bit(bit_index) { "1" } else { "0" }}
                            </div>
                        }
                    }).collect_view()}
                </div>

                {/* Input fields for different number representations */}
                <div class="content-wrapper">
                    <div class="bits-display">
                        <div class="number-repr">
                            <label>
                                DEC:
                                <input
                                    type="text"
                                    prop:value=dec_input
                                    on:input=input_dec
                                />
                            </label>
                            <label>
                                BIN:
                                <input
                                    type="text"
                                    prop:value=bin_input
                                    on:input=input_bin
                                />
                            </label>
                            <label>
                                HEX:
                                <input
                                    type="text"
                                    prop:value=hex_input
                                    on:input=input_hex
                                />
                            </label>
                            <label>
                                OCT:
                                <input
                                    type="text"
                                    prop:value=oct_input
                                    on:input=input_oct
                                />
                            </label>
                        </div>
                    </div>

                    {/* Buttons for bitwise operations */}
                    <div class="bit-operations">
                        <button class="bit-btn" on:click=lsh>"Lsh"</button>
                        <button class="bit-btn" on:click=rsh>"Rsh"</button>
                        <button class="bit-btn" on:click=lshr>"Lshr"</button>
                        <button class="bit-btn" on:click=rshr>"Rshr"</button>
                        <button class="bit-btn" on:click=not>"Not"</button>
                        <button class="bit-btn" on:click=clear>"Clr"</button>
                        <button class="bit-btn" on:click=set_all>"Set"</button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

/// The main function mounts the `App` component to the body of the document.
fn main() {
    mount_to_body(|| view! { <App/> })
}
