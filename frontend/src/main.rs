//! Bit Viewer Application
//!
//! This application provides interactive visualization and manipulation of 64-bit values
//! with support for multiple numeric representations and character encoding display.

use leptos::prelude::*;
use bit_operations::BitArray;
use hex;

/// Main application state and UI component
#[component]
fn App() -> impl IntoView {
    // Signal for storing and updating the 64-bit value
    let (bit_array, set_bit_array) = signal(BitArray::new());
    let (bit_size, set_bit_size) = signal(64);

    // Signals for input fields for different numeric representations
    let (dec_input, set_dec_input) = signal(String::new());
    let (bin_input, set_bin_input) = signal(String::new());
    let (hex_input, set_hex_input) = signal(String::new());
    let (hex_be_input, set_hex_be_input) = signal(String::new());
    let (hex_le_input, set_hex_le_input) = signal(String::new());
    let (oct_input, set_oct_input) = signal(String::new());
    let (ascii_input, set_ascii_input) = signal(String::new());
    let (utf8_input, set_utf8_input) = signal(String::new());

    // Calculate mask based on selected bit size to limit value range
    let mask = move || match bit_size.get() {
        8 => 0xFFu64,
        16 => 0xFFFFu64,
        32 => 0xFFFFFFFFu64,
        _ => u64::MAX,
    };

    // Effect to update all input fields when bit array changes
    Effect::new(move |_| {
        let current = bit_array.get().0 & mask();
        let le_bytes = current.to_le_bytes();
        let be_bytes = current.to_be_bytes();
        let byte_count = (bit_size.get() / 8) as usize;

        // Update numeric representations
        set_dec_input.set(current.to_string());
        set_bin_input.set(format!("0b{:0width$b}", current, width = bit_size.get()));
        set_hex_input.set(format!("0x{:X}", current));
        set_hex_be_input.set(format!("0x{}", hex::encode(&be_bytes[8 - byte_count..8])));
        set_hex_le_input.set(format!("0x{}", hex::encode(&le_bytes[0..byte_count])));
        set_oct_input.set(format!("0o{:o}", current));

        // Convert bytes to ASCII and UTF-8 representations
        let relevant_bytes_be = &be_bytes[8 - byte_count..8];
        let relevant_bytes_le = &le_bytes[0..byte_count];

        // Generate ASCII string (replace non-printable characters with space)
        let ascii_str: String = relevant_bytes_le.iter()
            .map(|&b| if (32..=126).contains(&b) { b as char } else { ' ' })
            .collect();

        // Generate UTF-8 string with validation
        let utf8_str = String::from_utf8_lossy(relevant_bytes_be).into_owned();

        // Handle partial UTF-8 sequences by replacing invalid parts
        let mut validated_utf8 = String::new();
        let mut chars = utf8_str.chars().peekable();
        while let Some(c) = chars.next() {
            if c.len_utf8() <= relevant_bytes_be.len() {
                validated_utf8.push(c);
            } else {
                // Add replacement character for incomplete sequences
                validated_utf8.push('\u{FFFD}');
                while let Some(next) = chars.peek() {
                    if next.len_utf8() > relevant_bytes_be.len() {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
        }

        set_utf8_input.set(if validated_utf8.is_empty() { " ".into() } else { validated_utf8 });
        set_ascii_input.set(ascii_str);
    });

    // Effect to update mask when bit size changes
    Effect::new(move |_| {
        set_bit_array.update(|ba| ba.0 &= mask());
    });

    // Universal value update handler with mask application
    let update_value = move |value: u64| {
        set_bit_array.set(BitArray(value & mask()));
    };

    // Input handlers for different numeric formats
    // --------------------------------------------------
    // Decimal input handler
    let input_dec = move |ev: web_sys::Event| {
        let val = event_target_value(&ev);
        set_dec_input.set(val.clone());
        if let Ok(num) = val.parse::<u64>() {
            update_value(num);
        }
    };

    // Binary input handler
    let input_bin = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev);
        val = val.replace("0b", "").replace(' ', "");
        set_bin_input.set(format!("0b{}", val));
        if let Ok(num) = u64::from_str_radix(&val, 2) {
            update_value(num);
        }
    };

    // Hexadecimal input handler (normal)
    let input_hex = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev);
        val = val.replace("0x", "").replace(' ', "");
        set_hex_input.set(format!("0x{}", val));

        // Truncate to maximum allowed digits for current bit size
        let max_digits = bit_size.get() / 4;
        if val.len() > max_digits as usize {
            val.truncate(max_digits as usize);
        }

        if let Ok(num) = u64::from_str_radix(&val, 16) {
            update_value(num);
        }
    };

    // Big-endian hexadecimal input handler
    let input_hex_be = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev);
        val = val.replace("0x", "").replace(' ', "");
        set_hex_be_input.set(format!("0x{}", val));

        let expected_len = (bit_size.get() / 4) as usize;
        if val.len() == expected_len {
            if let Ok(bytes) = hex::decode(&val) {
                // Reconstruct value from big-endian bytes
                let mut value = 0u64;
                for (i, &byte) in bytes.iter().enumerate() {
                    value |= (byte as u64) << ((bytes.len() - 1 - i) * 8);
                }
                update_value(value);
            }
        }
    };

    // Little-endian hexadecimal input handler
    let input_hex_le = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev);
        val = val.replace("0x", "").replace(' ', "");
        set_hex_le_input.set(format!("0x{}", val));

        let expected_len = (bit_size.get() / 4) as usize;
        if val.len() == expected_len {
            if let Ok(bytes) = hex::decode(&val) {
                // Reconstruct value from little-endian bytes
                let mut value = 0u64;
                for (i, &byte) in bytes.iter().enumerate() {
                    value |= (byte as u64) << (i * 8);
                }
                update_value(value);
            }
        }
    };

    // Octal input handler
    let input_oct = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev);
        val = val.replace("0o", "").replace(' ', "");
        set_oct_input.set(format!("0o{}", val));
        if let Ok(num) = u64::from_str_radix(&val, 8) {
            update_value(num);
        }
    };

    // Bitwise operation handlers
    // --------------------------------------------------
    // Logical shift left
    let lsh = move |_| set_bit_array.update(|ba| *ba = BitArray((ba.0 << 1) & mask()));
    // Logical shift right
    let rsh = move |_| set_bit_array.update(|ba| *ba = BitArray((ba.0 >> 1) & mask()));
    // Bitwise NOT
    let not = move |_| set_bit_array.update(|ba| *ba = BitArray((!ba.0) & mask()));
    // Clear all bits
    let clear = move |_| set_bit_array.set(BitArray(0));
    // Set all bits
    let set_all = move |_| set_bit_array.set(BitArray(mask()));
    // Left circular shift
    let lshr = move |_| {
        let size = bit_size.get();
        set_bit_array.update(|ba| {
            let rotated = (ba.0 << 1) | (ba.0 >> (size - 1));
            *ba = BitArray(rotated & mask());
        })
    };
    // Right circular shift
    let rshr = move |_| {
        let size = bit_size.get();
        set_bit_array.update(|ba| {
            let rotated = (ba.0 >> 1) | (ba.0 << (size - 1));
            *ba = BitArray(rotated & mask());
        })
    };

    // Bit size selector handler
    let update_bit_size = move |new_size: u64| {
        set_bit_size.set(new_size.try_into().unwrap());
    };

    // UI rendering
    view! {
        <div class="main-container">
            <div class="bit-size-selector">
                <span class="bit-size-label">Bit Size:</span>
                {[8, 16, 32, 64].into_iter().map(|size| view! {
                    <label>
                        <input
                            type="radio"
                            name="bit-size"
                            value=size
                            checked=move || bit_size.get() == size
                            on:change=move |_| update_bit_size(size.try_into().unwrap())
                        />
                        {size.to_string()}
                    </label>
                }).collect_view()}
            </div>

            // Bit grid visualization
            <div class="bit-grid">
                {(0..64).rev().map(|bit_index| {
                    let is_active = move || bit_index < bit_size.get();
                    view! {
                        <div
                            class="bit"
                            class:active=move || is_active() && bit_array.get().get_bit(bit_index as u8)
                            class:inactive=move || !is_active()
                            on:click=move |_| {
                                if is_active() {
                                    set_bit_array.update(|ba| ba.toggle_bit(bit_index as u8));
                                }
                            }
                            data-bit=bit_index
                        >
                            {move || if bit_array.get().get_bit(bit_index as u8) { "1" } else { "0" }}
                        </div>
                    }
                }).collect_view()}
            </div>

            // Input and controls section
            <div class="input-section">
                <div class="bits-display">
                    <div class="number-repr">
                        <label>DEC: <input type="text" prop:value=dec_input on:input=input_dec/></label>
                        <label>BIN: <input type="text" prop:value=bin_input on:input=input_bin/></label>
                        <label>HEX: <input type="text" prop:value=hex_input on:input=input_hex/></label>
                        <label>HEX BE: <input type="text" prop:value=hex_be_input on:input=input_hex_be/></label>
                        <label>HEX LE: <input type="text" prop:value=hex_le_input on:input=input_hex_le/></label>
                        <label>OCT: <input type="text" prop:value=oct_input on:input=input_oct/></label>
                        <label>ASCII: <input type="text" prop:value=ascii_input readonly/></label>
                        <label>UTF-8: <input type="text" prop:value=utf8_input readonly/></label>
                    </div>
                </div>

                // Bit operation buttons
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
    }
}

/// Main entry point
fn main() {
    mount_to_body(|| view! { <App/> });
}