#![no_main]

use libfuzzer_sys::fuzz_target;
use escp_layout::Region;

fuzz_target!(|data: &[u8]| {
    // We need at least 8 bytes for 4 u16 values
    if data.len() < 8 {
        return;
    }

    // Parse fuzzer input as u16 values
    let x = u16::from_le_bytes([data[0], data[1]]);
    let y = u16::from_le_bytes([data[2], data[3]]);
    let width = u16::from_le_bytes([data[4], data[5]]);
    let height = u16::from_le_bytes([data[6], data[7]]);

    // Test Region::new() - should never panic, only return Err
    let region_result = Region::new(x, y, width, height);

    // If region is valid, test operations on it
    if let Ok(region) = region_result {
        // Test split_vertical with various heights
        if data.len() >= 10 {
            let split_height = u16::from_le_bytes([data[8], data[9]]);
            let _ = region.split_vertical(split_height);
        }

        // Test split_horizontal with various widths
        if data.len() >= 12 {
            let split_width = u16::from_le_bytes([data[10], data[11]]);
            let _ = region.split_horizontal(split_width);
        }

        // Test with_padding with various padding values
        if data.len() >= 20 {
            let top = u16::from_le_bytes([data[12], data[13]]);
            let right = u16::from_le_bytes([data[14], data[15]]);
            let bottom = u16::from_le_bytes([data[16], data[17]]);
            let left = u16::from_le_bytes([data[18], data[19]]);
            let _ = region.with_padding(top, right, bottom, left);
        }

        // Test accessor methods (should never panic)
        let _ = region.x();
        let _ = region.y();
        let _ = region.width();
        let _ = region.height();
    }

    // Test full_page (should always succeed)
    let full = Region::full_page();
    let _ = full.x();
    let _ = full.y();
    let _ = full.width();
    let _ = full.height();
});
