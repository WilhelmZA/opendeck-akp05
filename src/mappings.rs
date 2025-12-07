use mirajazz::{
    device::DeviceQuery,
    types::{HidDeviceInfo, ImageFormat, ImageMirroring, ImageMode, ImageRotation},
};

// Must match DeviceNamespace field in manifest.json
pub const DEVICE_NAMESPACE: &str = "a5";

// AKP05E series constants - custom layout: 10 physical buttons + 4 touchscreen zones
pub const ROW_COUNT: usize = 2;  // 2 rows of physical buttons only
pub const COL_COUNT: usize = 5;  // 5 columns for physical buttons
pub const KEY_COUNT: usize = 10; // Back to 10 physical buttons for main grid
pub const ENCODER_COUNT: usize = 4;

#[derive(Debug, Clone)]
pub enum Kind {
    Akp05E,  // AKP05E variant
    // Future AKP05 variants (AKP05F, AKP05G, etc.) can be added here
}

pub const AJAZZ_VID: u16 = 0x0300;
pub const AKP03E_REV2_PID: u16 = 0x3002;   // Original working PID
pub const AKP05E_PID: u16 = 0x3004;        // Your actual device PID

// Try both the original working PID and your actual device PID
pub const AKP05E_QUERY_REV2: DeviceQuery = DeviceQuery::new(65440, 1, AJAZZ_VID, AKP03E_REV2_PID);
pub const AKP05E_QUERY: DeviceQuery = DeviceQuery::new(65440, 1, AJAZZ_VID, AKP05E_PID);

pub const QUERIES: [DeviceQuery; 2] = [
    AKP05E_QUERY_REV2,  // Try original working PID first
    AKP05E_QUERY,       // Then try your actual PID
];

impl Kind {
    /// Returns the number of rows for this device
    pub fn row_count(&self) -> usize {
        ROW_COUNT // AKP05E has 3 rows: 2 button rows + 1 encoder row
    }

    /// Returns the number of columns for this device
    pub fn col_count(&self) -> usize {
        COL_COUNT // AKP05E has 5 columns max (button rows), encoder row has 4
    }

    /// Returns the total number of keys for this device
    pub fn key_count(&self) -> usize {
        KEY_COUNT // All AKP05E devices have 10 keys
    }

    /// Returns the number of encoders for this device
    pub fn encoder_count(&self) -> usize {
        ENCODER_COUNT // All AKP05E devices have 4 encoders
    }

    /// Matches devices VID+PID pairs to correct kinds
    pub fn from_vid_pid(vid: u16, pid: u16) -> Option<Self> {
        match vid {
            AJAZZ_VID => match pid {
                AKP03E_REV2_PID => Some(Kind::Akp05E),  // Treat 0x3002 as AKP05E
                AKP05E_PID => Some(Kind::Akp05E),       // Treat 0x3004 as AKP05E  
                _ => None,
            },
            _ => None,
        }
    }

    /// Maps software button index to physical device button index
    pub fn map_button_index(&self, software_index: usize) -> usize {
        match self {
            Self::Akp05E => {
                match software_index {
                    // Software 0-3 -> Physical 10-14 (encoders)
                    0 => 10, 1 => 11, 2 => 12, 3 => 13, 4 => 14,
                    // Software 5-8 -> Physical 5-9 (middle row)
                    5 => 5, 6 => 6, 7 => 7, 8 => 8, 9 => 9,
                    // Software 10-14 -> Physical 0-4 (top row)  
                    10 => 0, 11 => 1, 12 => 2, 13 => 3, 14 => 4,
                    // Invalid index - panic
                    _ => panic!("Invalid software index: {}", software_index),
                }   
            }
        }
    }

    /// Returns human-readable device name
    pub fn human_name(&self) -> String {
        match &self {
            Self::Akp05E => "Ajazz AKP05E",
        }
        .to_string()
    }

    /// Returns protocol version for device
    pub fn protocol_version(&self) -> usize {
        3 // All AKP05E devices use protocol version 3
    }

    /// Returns image format configuration
    pub fn image_format(&self) -> ImageFormat {
        ImageFormat {
            mode: ImageMode::JPEG,
            size: (120, 120),
            rotation: ImageRotation::Rot180,  // 90 degrees more from Rot90
            mirror: ImageMirroring::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CandidateDevice {
    pub id: String,
    pub dev: HidDeviceInfo,
    pub kind: Kind,
}
