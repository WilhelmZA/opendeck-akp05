use mirajazz::{error::MirajazzError, types::DeviceInput};

use crate::mappings::{ENCODER_COUNT, KEY_COUNT};

// Simplified input processing for AKP05 devices only
pub fn process_input(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    // All supported devices are AKP05 variants, so use AKP05E processing
    process_akp05e_input(input, state)
}

fn process_akp05e_input(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    match input {
        // 10 buttons for AKP05E (1-10, using 1-based indexing)
        0x01..=0x0A => read_akp05e_button_press(input, state),
        // Primary encoder rotations
        0x90 | 0x91 | 0x50 | 0x51 | 0x60 | 0x61 | 0x70 | 0x71 => read_akp05e_encoder_value(input),
        // Additional encoder 1 rotations (knob 1)
        0xA0 | 0xA1 => read_akp05e_encoder_value_alt(input),
        // Encoder button presses (including the new knob 1 click)
        0x33..=0x37 => read_akp05e_encoder_press(input, state),
        // Touchscreen inputs (ignore for now, but don't error)
        0x40..=0x4F => read_akp05e_touchscreen(input, state),
        // Unknown inputs - silently ignore to prevent disconnections
        _ => {
            // Return empty state change instead of error to prevent disconnections
            Ok(DeviceInput::ButtonStateChange(vec![false; 10]))
        },
    }
}

// AKP05E button state reading
fn read_button_states(states: &[u8]) -> Vec<bool> {
    let mut bools = vec![];

    for i in 0..KEY_COUNT {
        bools.push(states[i + 1] != 0);
    }

    bools
}

// AKP05E button press handling
fn read_akp05e_button_press(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    // Convert 1-based input (0x01-0x0A) to 0-based physical button index (0-9)
    let physical_button = (input - 1) as usize;
    
    if physical_button >= KEY_COUNT {
        log::warn!("Physical button {} out of range (max {})", physical_button, KEY_COUNT - 1);
        return Err(MirajazzError::BadData);
    }

    // For button presses, use 1:1 mapping - physical button equals UI position
    let ui_position = physical_button;

    let mut button_states = vec![0x01];
    button_states.extend(vec![0u8; KEY_COUNT + 1]); // 10 buttons + 1
    
    button_states[ui_position + 1] = state;

    Ok(DeviceInput::ButtonStateChange(read_button_states(
        &button_states,
    )))
}

// AKP05E encoder value handling  
fn read_akp05e_encoder_value(input: u8) -> Result<DeviceInput, MirajazzError> {
    let mut encoder_values = vec![0i8; ENCODER_COUNT]; // AKP05E has 4 encoders

    let (encoder, value): (usize, i8) = match input {
        // Encoder 1 (primary codes)
        0x30 => (0, -1), // encoder 1 left
        0x31 => (0, 1),  // encoder 1 right
        // Encoder 2 (from your testing: 0x50/0x51)
        0x50 => (1, -1), // encoder 2 left
        0x51 => (1, 1),  // encoder 2 right
        // Encoder 3 (from your testing: 0x90/0x91)
        0x90 => (2, -1), // encoder 3 left
        0x91 => (2, 1),  // encoder 3 right
        // Encoder 4 (needs testing)
        0x70 => (3, -1), // encoder 4 left
        0x71 => (3, 1),  // encoder 4 right
        _ => return Err(MirajazzError::BadData),
    };

    encoder_values[encoder] = value;
    Ok(DeviceInput::EncoderTwist(encoder_values))
}

// Alternative encoder mappings (encoder 1 has alternate codes)
fn read_akp05e_encoder_value_alt(input: u8) -> Result<DeviceInput, MirajazzError> {
    let mut encoder_values = vec![0i8; ENCODER_COUNT]; // AKP05E has 4 encoders

    let (encoder, value): (usize, i8) = match input {
        // Encoder 1 alternate mappings (from your testing)
        0xA0 => (0, -1), // encoder 1 left
        0xA1 => (0, 1),  // encoder 1 right
        _ => {
            log::warn!("Unknown alternative encoder input: 0x{:02X}", input);
            return Ok(DeviceInput::ButtonStateChange(vec![false; 10]));
        }
    };

    encoder_values[encoder] = value;
    Ok(DeviceInput::EncoderTwist(encoder_values))
}

// Touchscreen input handler - just log for now
fn read_akp05e_touchscreen(_input: u8, _state: u8) -> Result<DeviceInput, MirajazzError> {
    // Touchscreen input detected but not implemented for UI interaction
    
    // Return empty state to avoid interfering with main button layout
    Ok(DeviceInput::ButtonStateChange(vec![false; 10]))
}

// AKP05E encoder press handling (corrected based on testing)
fn read_akp05e_encoder_press(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    let mut encoder_states = vec![false; ENCODER_COUNT]; // AKP05E has 4 encoders

    let encoder: usize = match input {
        0x37 => 0, // Knob 1 click 
        0x35 => 1, // Knob 2 click
        0x33 => 2, // Knob 3 click
        0x36 => 3, // Knob 4 click
        _ => {
            log::warn!("Unknown encoder button: 0x{:02X}", input);
            return Ok(DeviceInput::ButtonStateChange(vec![false; 10]));
        }
    };

    encoder_states[encoder] = state != 0;
    Ok(DeviceInput::EncoderStateChange(encoder_states))
}
