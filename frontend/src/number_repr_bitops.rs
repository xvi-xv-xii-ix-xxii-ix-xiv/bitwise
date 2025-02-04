//! Number Representations and Bit Operations Module

use leptos::*;
use leptos::prelude::*;
use bit_operations::BitArray;
use hex;
use web_sys::MouseEvent;

/// Combined number representations and bit operations component
#[component]
pub fn NumberReprAndBitOps(
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

    let current_value = move || bit_array.get().0 & mask();

    // Основные исправления:
    // 1. Удалены статические переменные и Mutex
    // 2. Исправлены обработчики HEX BE/LE
    // 3. Добавлена корректная обработка форматов
    // 4. Исправлены битовые операции

    // Input handlers
    let input_dec = move |ev: web_sys::Event| {
        let val = event_target_value(&ev)
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();

        let num = val.parse().unwrap_or(0);
        set_bit_array.set(BitArray(num & mask()));
    };

    let input_bin = move |ev: web_sys::Event| {
        let val = event_target_value(&ev)
            .replace("0b", "")
            .chars()
            .filter(|c| *c == '0' || *c == '1')
            .collect::<String>();

        let num = u64::from_str_radix(&val, 2).unwrap_or(0);
        set_bit_array.set(BitArray(num & mask()));
    };

    let input_hex = move |ev: web_sys::Event| {
        let val = event_target_value(&ev)
            .replace("0x", "")
            .to_uppercase()
            .chars()
            .filter(|c| c.is_ascii_hexdigit())
            .collect::<String>();

        let num = u64::from_str_radix(&val, 16).unwrap_or(0);
        set_bit_array.set(BitArray(num & mask()));
    };

    let input_hex_be = move |ev: web_sys::Event| {
        let val = event_target_value(&ev)
            .replace("0x", "")
            .to_uppercase()
            .chars()
            .filter(|c| c.is_ascii_hexdigit())
            .collect::<String>();

        let byte_count = (bit_size.get() / 8) as usize;
        let padded_val = format!("{:0>width$}", val, width = byte_count * 2);

        if let Ok(bytes) = hex::decode(&padded_val) {
            let value = bytes.iter().fold(0u64, |acc, &b| (acc << 8) | b as u64);
            set_bit_array.set(BitArray(value & mask()));
        }
    };

    let input_hex_le = move |ev: web_sys::Event| {
        let val = event_target_value(&ev)
            .replace("0x", "")
            .to_uppercase()
            .chars()
            .filter(|c| c.is_ascii_hexdigit())
            .collect::<String>();

        let byte_count = (bit_size.get() / 8) as usize;
        let padded_val = format!("{:0>width$}", val, width = byte_count * 2);

        if let Ok(bytes) = hex::decode(&padded_val) {
            let value = bytes.iter().rev().fold(0u64, |acc, &b| (acc << 8) | b as u64);
            set_bit_array.set(BitArray(value & mask()));
        }
    };

    let input_oct = move |ev: web_sys::Event| {
        let val = event_target_value(&ev)
            .replace("0o", "")
            .chars()
            .filter(|c| ('0'..='7').contains(c))
            .collect::<String>();

        let num = u64::from_str_radix(&val, 8).unwrap_or(0);
        set_bit_array.set(BitArray(num & mask()));
    };

    // Bit operations with explicit closure type
    let bit_ops = vec![
        ("Lsh", Box::new(move |_: MouseEvent| set_bit_array.update(|ba| ba.0 = (ba.0 << 1) & mask())) as Box<dyn Fn(MouseEvent) + Send + Sync>),
        ("Rsh", Box::new(move |_: MouseEvent| set_bit_array.update(|ba| ba.0 = (ba.0 >> 1) & mask())) as Box<dyn Fn(MouseEvent) + Send + Sync>),
        ("Lshr", Box::new(move |_: MouseEvent| set_bit_array.update(|ba| ba.0 = (ba.0.rotate_left(1)) & mask())) as Box<dyn Fn(MouseEvent) + Send + Sync>),
        ("Rshr", Box::new(move |_: MouseEvent| set_bit_array.update(|ba| ba.0 = (ba.0.rotate_right(1)) & mask())) as Box<dyn Fn(MouseEvent) + Send + Sync>),
        ("Not", Box::new(move |_: MouseEvent| set_bit_array.update(|ba| ba.0 = (!ba.0) & mask())) as Box<dyn Fn(MouseEvent) + Send + Sync>),
        ("Clr", Box::new(move |_: MouseEvent| set_bit_array.set(BitArray(0))) as Box<dyn Fn(MouseEvent) + Send + Sync>),
        ("Set", Box::new(move |_: MouseEvent| set_bit_array.set(BitArray(mask()))) as Box<dyn Fn(MouseEvent) + Send + Sync>),
    ];

    // Character representations
    let byte_count = move || (bit_size.get() / 8) as usize;
    let ascii_str = move || {
        let bytes = current_value().to_le_bytes();
        String::from_utf8_lossy(&bytes[..byte_count()])
            .chars()
            .map(|c| if c.is_ascii_graphic() { c } else { ' ' })
            .collect::<Vec<_>>()

    };

    let utf8_str = move || {
        let bytes = current_value().to_be_bytes();
        String::from_utf8_lossy(&bytes[8 - byte_count()..8])
            .into_owned()
    };

    view! {
            <div class="number-repr">
                <label><span class="input-label">DEC</span>
                    <input type="text"
                        value=move || current_value().to_string()
                        on:input=input_dec
                    />
                </label>
                <label><span class="input-label">BIN</span>
                    <input type="text"
                        value=move || format!("0b{:0width$b}", current_value(), width=bit_size.get() as usize)
                        on:input=input_bin
                    />
                </label>
                <label><span class="input-label">HEX</span>
                    <input type="text"
                        value=move || format!("0x{:X}", current_value())
                        on:input=input_hex
                    />
                </label>
                <label><span class="input-label">HEX BE</span>
                    <input type="text"
                        value=move || format!("0x{:0width$X}", current_value(), width=(bit_size.get()/4) as usize)
                        on:input=input_hex_be
                    />
                </label>
                <label><span class="input-label">HEX LE</span>
                    <input type="text"
                        value=move || format!("0x{:0width$X}", current_value(), width=(bit_size.get()/4) as usize)
                        on:input=input_hex_le
                    />
                </label>
                <label><span class="input-label">OCT</span>
                    <input type="text"
                        value=move || format!("0o{:o}", current_value())
                        on:input=input_oct
                    />
                </label>
                <label><span class="input-label">ASCII</span>
                    <input type="text" readonly value=ascii_str()/>
                </label>
                <label><span class="input-label">UTF-8</span>
                    <input type="text" readonly value=utf8_str()/>
                </label>
            </div>

            <div class="special-generator">
                <div class="bit-operations">
                    {bit_ops.into_iter().map(|(name, handler)| view! {
                        <button class="bit-btn" on:click=handler>
                            {name}
                        </button>
                    }).collect_view()}
                </div>
            </div>

    }
}