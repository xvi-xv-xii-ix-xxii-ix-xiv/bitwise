use bit_operations::BitArray;

/// Toggles a specific bit in the given 64-bit integer at the specified position.
///
/// # Arguments
/// * `bit_array` - The original 64-bit integer representing the bit array.
/// * `position` - The position of the bit to toggle (0-based index).
///
/// # Returns
/// A new 64-bit integer with the bit at `position` toggled.
#[tauri::command]
fn toggle_bit(bit_array: u64, position: u8) -> u64 {
    let mut ba = BitArray(bit_array);
    ba.toggle_bit(position);
    ba.0
}

/// Retrieves all bits from a 64-bit integer as a vector of booleans.
///
/// # Arguments
/// * `state` - The 64-bit integer representing the bit array.
///
/// # Returns
/// A vector of booleans representing the state of each bit.
#[tauri::command]
fn get_bits(state: u64) -> Vec<bool> {
    let bits = BitArray(state);
    bits.get_all_bits()
}

/// Returns the raw value of the given 64-bit integer.
///
/// # Arguments
/// * `state` - The 64-bit integer.
///
/// # Returns
/// The raw 64-bit integer value.
#[tauri::command]
fn get_raw(state: u64) -> u64 {
    state
}

/// Processes the bits of the given 64-bit integer. Currently, it reverses the bit order.
///
/// # Arguments
/// * `bits` - The 64-bit integer to process.
///
/// # Returns
/// A new 64-bit integer with the bits reversed.
#[tauri::command]
fn process_bits(bits: u64) -> u64 {
    // Bit processing logic: reverse the bits.
    bits.reverse_bits()
}

/// The main function initializes and runs the Tauri application.
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![toggle_bit, get_bits, get_raw, process_bits])
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
