//! Bit Viewer Application
//!
//! This application provides interactive visualization and manipulation of 64-bit values
//! with support for multiple numeric representations, character encoding display,
//! IEEE 754 decoding, number distribution visualization, and special value generation.

mod bit_grid;
mod ieee754;
mod plot;
mod special_values;
// mod number_repr_bitops;

use crate::plot::DistributionPlot;
use bit_grid::BitGrid;
use bit_operations::BitArray;
use hex;
use ieee754::IEEE754Display;
use leptos::prelude::*;
use leptos::*;
use special_values::SpecialValueGenerator;
use std::cmp::PartialEq;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum InputMode {
    Integer,
    Float,
}

/// Main application state and UI component
#[component]
fn App() -> impl IntoView {
    // Signal for storing and updating the 64-bit value
    let (bit_array, set_bit_array) = signal(BitArray::new());
    let (bit_size, set_bit_size): (ReadSignal<u64>, WriteSignal<u64>) = signal(64);

    // Signals for input fields
    let (input_mode, set_input_mode) = signal(InputMode::Integer);
    let (dec_input, set_dec_input) = signal(String::new());
    let (bin_input, set_bin_input) = signal(String::new());
    let (hex_input, set_hex_input) = signal(String::new());
    let (hex_be_input, set_hex_be_input) = signal(String::new());
    let (hex_le_input, set_hex_le_input) = signal(String::new());
    let (oct_input, set_oct_input) = signal(String::new());
    let (ascii_input, set_ascii_input) = signal(String::new());
    let (utf8_input, set_utf8_input) = signal(String::new());

    // Calculate mask based on selected bit size
    let mask = move || match bit_size.get() {
        8 => 0xFFu64,
        16 => 0xFFFFu64,
        32 => 0xFFFFFFFFu64,
        _ => u64::MAX,
    };

    // Effect to update all fields when bit array changes
    Effect::new(move |_| {
        let current = bit_array.get().0 & mask();
        let le_bytes = current.to_le_bytes();
        let be_bytes = current.to_be_bytes();
        let byte_count = (bit_size.get() / 8) as usize;

        if input_mode.get() == InputMode::Integer {
            set_dec_input.set(current.to_string()); // Обычное целое число
        } else {
            let float_value = match bit_size.get() {
                16 => half::f16::from_bits(current as u16).to_f64(),
                32 => f32::from_bits((current & 0xFFFFFFFF) as u32) as f64,
                64 => f64::from_bits(current),
                _ => 0.0,
            };
            // set_dec_input.set(float_value.to_string());
        }

        // Update numeric representations
        set_bin_input.set(format!(
            "0b{:0width$b}",
            current,
            width = bit_size.get() as usize
        ));
        set_hex_input.set(format!("0x{:X}", current));
        set_hex_be_input.set(format!("0x{}", hex::encode(&be_bytes[8 - byte_count..8])));
        set_hex_le_input.set(format!("0x{}", hex::encode(&le_bytes[0..byte_count])));
        set_oct_input.set(format!("0o{:o}", current));

        // Update character representations
        let relevant_bytes_le = &le_bytes[0..byte_count];
        let ascii_str: String = relevant_bytes_le
            .iter()
            .map(|&b| {
                if (32..=126).contains(&b) {
                    b as char
                } else {
                    ' '
                }
            })
            .collect();
        set_ascii_input.set(ascii_str);

        let utf8_str = String::from_utf8_lossy(&be_bytes[8 - byte_count..8]).into_owned();
        set_utf8_input.set(if utf8_str.is_empty() {
            " ".into()
        } else {
            utf8_str
        });
    });

    // Effect to apply bit size mask
    Effect::new(move |_| {
        let current_mask = mask();
        set_bit_array.update(|ba| ba.0 &= current_mask);
    });

    // Universal value updater
    let update_value = move |value: u64| {
        set_bit_array.set(BitArray(value & mask()));
    };

    // Input handlers with validation
    let input_dec = move |ev: web_sys::Event| {
        let input = event_target_value(&ev);

        if input_mode.get() == InputMode::Integer {
            // Целочисленный режим: только цифры
            let filtered = input
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();
            set_dec_input.set(filtered.clone());
            if let Ok(num) = filtered.parse::<u64>() {
                update_value(num);
            }
        } else {
            // Режим с плавающей точкой: разрешаем цифры, точку, экспоненту, знаки
            let mut has_point = false;
            let mut has_exponent = false;
            let mut filtered_chars = Vec::new();

            for (i, c) in input.chars().enumerate() {
                match c {
                    '-' => {
                        // Разрешаем минус в начале или после экспоненты
                        if i == 0
                            || (has_exponent
                                && filtered_chars
                                    .last()
                                    .map_or(false, |&lc| lc == 'e' || lc == 'E'))
                        {
                            filtered_chars.push(c);
                        }
                    }
                    '+' => {
                        // Разрешаем плюс только после экспоненты
                        if has_exponent
                            && filtered_chars
                                .last()
                                .map_or(false, |&lc| lc == 'e' || lc == 'E')
                        {
                            filtered_chars.push(c);
                        }
                    }
                    '.' => {
                        // Точка разрешена до экспоненты и только одна
                        if !has_point && !has_exponent {
                            has_point = true;
                            filtered_chars.push(c);
                        }
                    }
                    'e' | 'E' => {
                        // Экспонента разрешена один раз, если есть цифры до
                        if !has_exponent && !filtered_chars.is_empty() {
                            has_exponent = true;
                            filtered_chars.push(c);
                        }
                    }
                    c if c.is_ascii_digit() => filtered_chars.push(c),
                    _ => (),
                }
            }

            let filtered = filtered_chars.into_iter().collect::<String>();
            set_dec_input.set(filtered.clone());

            // Автодополнение ведущего нуля для точек
            let filtered = if filtered.starts_with('.') {
                format!("0{}", filtered)
            } else if filtered.is_empty() {
                "0".to_string()
            } else {
                filtered
            };

            // Парсим и обновляем биты
            if let Ok(num) = filtered.parse::<f64>() {
                let bits = match bit_size.get() {
                    16 => u64::from(half::f16::from_f64(num).to_bits() as u16),
                    32 => u64::from((num as f32).to_bits() as u32),
                    64 => num.to_bits(),
                    _ => 0,
                };
                set_bit_array.set(BitArray(bits));
            }
        }
    };

    let input_bin = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev).replace("0b", "").replace(' ', "");
        val = val.chars().filter(|c| *c == '0' || *c == '1').collect();
        let filtered = if val.is_empty() { "0" } else { &val };
        set_bin_input.set(format!("0b{}", filtered));
        if let Ok(num) = u64::from_str_radix(filtered, 2) {
            update_value(num);
        }
    };

    let input_hex = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev)
            .replace("0x", "")
            .replace(' ', "")
            .to_uppercase();
        val = val.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        let max_len = bit_size.get() / 4;
        val.truncate(max_len as usize);
        let filtered = if val.is_empty() { "0" } else { &val };
        set_hex_input.set(format!("0x{}", filtered));
        if let Ok(num) = u64::from_str_radix(filtered, 16) {
            update_value(num);
        }
    };

    let input_hex_be = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev)
            .replace("0x", "")
            .replace(' ', "")
            .to_uppercase();
        val = val.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        let expected_len = (bit_size.get() / 4) as usize;
        val.truncate(expected_len);
        set_hex_be_input.set(format!("0x{}", val));
        if val.len() == expected_len {
            if let Ok(bytes) = hex::decode(&val) {
                let value = bytes.iter().enumerate().fold(0u64, |acc, (i, &b)| {
                    acc | (b as u64) << ((bytes.len() - 1 - i) * 8)
                });
                update_value(value);
            }
        }
    };

    let input_hex_le = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev)
            .replace("0x", "")
            .replace(' ', "")
            .to_uppercase();
        val = val.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        let expected_len = (bit_size.get() / 4) as usize;
        val.truncate(expected_len);
        set_hex_le_input.set(format!("0x{}", val));
        if val.len() == expected_len {
            if let Ok(bytes) = hex::decode(&val) {
                let value = bytes
                    .iter()
                    .enumerate()
                    .fold(0u64, |acc, (i, &b)| acc | (b as u64) << (i * 8));
                update_value(value);
            }
        }
    };

    let input_oct = move |ev: web_sys::Event| {
        let mut val = event_target_value(&ev).replace("0o", "").replace(' ', "");
        val = val.chars().filter(|c| ('0'..='7').contains(c)).collect();
        let filtered = if val.is_empty() { "0" } else { &val };
        set_oct_input.set(format!("0o{}", filtered));
        if let Ok(num) = u64::from_str_radix(filtered, 8) {
            update_value(num);
        }
    };

    // Bit operations
    let lsh = move |_| set_bit_array.update(|ba| *ba = BitArray((ba.0 << 1) & mask()));
    let rsh = move |_| set_bit_array.update(|ba| *ba = BitArray((ba.0 >> 1) & mask()));
    let not = move |_| set_bit_array.update(|ba| *ba = BitArray((!ba.0) & mask()));
    let clear = move |_| set_bit_array.set(BitArray(0));
    let set_all = move |_| set_bit_array.set(BitArray(mask()));
    let lshr = move |_| {
        set_bit_array.update(|ba| {
            let size = bit_size.get();
            *ba = BitArray((ba.0 << 1 | ba.0 >> (size - 1)) & mask())
        })
    };
    let rshr = move |_| {
        set_bit_array.update(|ba| {
            let size = bit_size.get();
            *ba = BitArray((ba.0 >> 1 | ba.0 << (size - 1)) & mask())
        })
    };

    // Bit size selector
    let update_bit_size = move |new_size: u64| {
        set_bit_size.set(new_size);
    };

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
                            on:change=move |_| update_bit_size(size)
                        />
                        {size.to_string()}
                    </label>
                }).collect_view()}
                    <label class="float-mode">
                <input
                    type="checkbox"
                    checked=move || matches!(input_mode.get(), InputMode::Float)
                    on:change=move |ev| {
                        let checked = event_target_checked(&ev);
                        set_input_mode.set(if checked {
                            InputMode::Float
                        } else {
                            InputMode::Integer
                        });
                    }
                />
                "Float"
            </label>
            </div>

            <div class="decoder-generator-container">
                <BitGrid bit_array=bit_array set_bit_array=set_bit_array bit_size=bit_size />
            </div>

            <div class="decoder-generator-container">
                <div class="number-repr">
                    <label>
                        <span class="input-label">DEC</span>
                        <input type="text" prop:value=dec_input on:input=input_dec/>
                    </label>
                    <label>
                        <span class="input-label">BIN</span>
                        <input type="text" prop:value=bin_input on:input=input_bin/>
                    </label>
                    <label>
                        <span class="input-label">HEX</span>
                        <input type="text" prop:value=hex_input on:input=input_hex/>
                    </label>
                    <label>
                        <span class="input-label">HEX BE</span>
                        <input type="text" prop:value=hex_be_input on:input=input_hex_be/>
                    </label>
                    <label>
                        <span class="input-label">HEX LE</span>
                        <input type="text" prop:value=hex_le_input on:input=input_hex_le/>
                    </label>
                    <label>
                        <span class="input-label">OCT</span>
                        <input type="text" prop:value=oct_input on:input=input_oct/>
                    </label>
                    <label>
                        <span class="input-label">ASCII</span>
                        <input type="text" prop:value=ascii_input readonly/>
                    </label>
                    <label>
                        <span class="input-label">UTF-8</span>
                        <input type="text" prop:value=utf8_input readonly/>
                    </label>
                </div>

                <div class="special-generator">
                    <label>
                        <span class="input-label">Bit operations</span>
                    </label>
                    <div class="bit-operations">
                        <button class="bit-btn" on:click=lsh>"Lsh"</button>
                        <button class="bit-btn" on:click=rsh>"Rsh"</button>
                        <button class="bit-btn" on:click=lshr>"Lshr"</button>
                        <button class="bit-btn" on:click=rshr>"Rshr"</button>
                        <button class="bit-btn" on:click=not>"Not"</button>
                        <button class="bit-btn" on:click=clear>"Clr"</button>
                        <button class="bit-btn" on:click=set_all>"Set"</button>
                    </div>
                     <SpecialValueGenerator set_bit_array=set_bit_array bit_size=bit_size />
                </div>
            </div>

            <div class="decoder-generator-container">
                <IEEE754Display bit_array=bit_array bit_size=bit_size />
            </div>
            <div class="input-operations-container">
                <DistributionPlot bit_array=bit_array.into() bit_size=bit_size/>
            </div>

        </div>
    }
}

/// Main entry point
fn main() {
    mount_to_body(|| view! { <App/> });
}
