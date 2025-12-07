use mirajazz::{error::MirajazzError, types::DeviceInput};

use crate::mappings::{ENCODER_COUNT, KEY_COUNT};

// Simplified input processing for AKP05 devices only
pub fn process_input(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    log::info!("Processing input: {}, {}", input, state);
    // All supported devices are AKP05 variants, so use AKP05E processing
    process_akp05e_input(input, state)
}

fn process_akp05e_input(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    match input {
        // 10 buttons for AKP05E (0-9)
        (0..=9) => read_akp05e_button_press(input, state),
        // 4 encoders
        0x90 | 0x91 | 0x50 | 0x51 | 0x60 | 0x61 | 0x70 | 0x71 => read_akp05e_encoder_value(input),
        0x33..=0x36 => read_akp05e_encoder_press(input, state),
        _ => Err(MirajazzError::BadData),
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
    let mut button_states = vec![0x01];
    button_states.extend(vec![0u8; KEY_COUNT + 1]); // 10 buttons + 1

    if input == 0 {
        return Ok(DeviceInput::ButtonStateChange(read_button_states(
            &button_states,
        )));
    }

    if input > 9 {
        return Err(MirajazzError::BadData);
    }

    button_states[input as usize + 1] = state;

    Ok(DeviceInput::ButtonStateChange(read_button_states(
        &button_states,
    )))
}

// AKP05E encoder value handling
fn read_akp05e_encoder_value(input: u8) -> Result<DeviceInput, MirajazzError> {
    let mut encoder_values = vec![0i8; ENCODER_COUNT]; // AKP05E has 4 encoders

    let (encoder, value): (usize, i8) = match input {
        // First encoder
        0x90 => (0, -1),
        0x91 => (0, 1),
        // Second encoder
        0x50 => (1, -1),
        0x51 => (1, 1),
        // Third encoder
        0x60 => (2, -1),
        0x61 => (2, 1),
        // Fourth encoder
        0x70 => (3, -1),
        0x71 => (3, 1),
        _ => return Err(MirajazzError::BadData),
    };

    encoder_values[encoder] = value;
    Ok(DeviceInput::EncoderTwist(encoder_values))
}

// AKP05E encoder press handling
fn read_akp05e_encoder_press(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    let mut encoder_states = vec![false; ENCODER_COUNT]; // AKP05E has 4 encoders

    let encoder: usize = match input {
        0x33 => 0, // First encoder
        0x34 => 1, // Second encoder
        0x35 => 2, // Third encoder
        0x36 => 3, // Fourth encoder
        _ => return Err(MirajazzError::BadData),
    };

    encoder_states[encoder] = state != 0;
    Ok(DeviceInput::EncoderStateChange(encoder_states))
}
