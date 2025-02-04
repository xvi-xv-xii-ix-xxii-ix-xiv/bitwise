//! Number Distribution Plot Module

use super::ieee754::IEEEDecoder;
use bit_operations::BitArray;
use leptos::prelude::*;

/// Plot Position Calculator
pub fn calculate_plot_position(bits: u64, bit_size: u64) -> f64 {
    let decoder = IEEEDecoder::new(bits, bit_size);
    let value = decoder.value;

    match decoder.special.as_str() {
        "NaN" => return 50.0,
        "+Inf" => return 100.0,
        "-Inf" => return 0.0,
        "Zero" => return 50.0,
        _ => (),
    }

    let is_negative = value.is_sign_negative();
    let abs_value = value.abs();

    match decoder.special.as_str() {
        "Denormalized" => {
            let min_normal = 2.0f64.powi(match decoder.format {
                super::ieee754::IEEEFormat::Half => -14,
                super::ieee754::IEEEFormat::Single => -126,
                super::ieee754::IEEEFormat::Double => -1022,
            });

            let ratio = abs_value / min_normal;
            if is_negative {
                30.0 * ratio
            } else {
                70.0 + 30.0 * ratio
            }
        }
        "Normalized" => {
            let (min_exp, max_exp) = match decoder.format {
                super::ieee754::IEEEFormat::Half => (-14, 15),
                super::ieee754::IEEEFormat::Single => (-126, 127),
                super::ieee754::IEEEFormat::Double => (-1022, 1023),
            };

            let log_val = abs_value.log2().clamp(min_exp as f64, max_exp as f64);
            let range = (max_exp - min_exp) as f64;
            let normalized = (log_val - min_exp as f64) / range * 60.0 + 20.0;

            if is_negative {
                40.0 - normalized
            } else {
                60.0 + normalized
            }
        }
        _ => 50.0,
    }
}

/// Distribution Plot Component
#[component]
pub fn DistributionPlot(
    bit_array: ReadSignal<BitArray>,
    bit_size: ReadSignal<u64>,
) -> impl IntoView {
    let position = move || {
        format!(
            "{}%",
            calculate_plot_position(bit_array.get().0, bit_size.get()).clamp(0.0, 100.0)
        )
    };

    view! {
        <div class="distribution-plot">
            <div class="plot-axis">
                <div class="zone negative-inf"></div>
                <div class="zone negative-subnormal"></div>
                <div class="zone negative-normal"></div>
                <div class="zone positive-normal"></div>
                <div class="zone positive-subnormal"></div>
                <div class="zone positive-inf"></div>

                <div class="plot-marker" style:left=position>
                    <div class="plot-tooltip">
                        {move || {
                            let _decoder = IEEEDecoder::new(bit_array.get().0, 64);

                        }}
                    </div>
                </div>
            </div>
            <div class="plot-labels">
                <span class="label-left">-Inf</span>
                <span class="label-subnormal">Subnormal</span>
                <span class="label-normal">Normal</span>
                <span class="label-zero">0</span>
                <span class="label-normal">Normal</span>
                <span class="label-subnormal">Subnormal</span>
                <span class="label-right">Inf</span>
            </div>
        </div>
    }
}
