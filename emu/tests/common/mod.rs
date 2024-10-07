use std::{fs, path::PathBuf};

pub fn setup_logger() {
    win_dbg_logger::init();
    log::set_max_level(log::LevelFilter::Debug);
}

pub fn serialize_tv_buffer<const SCANLINES: usize, const PIXELS_PER_SCANLINE: usize>(
    buffer: &[[u8; PIXELS_PER_SCANLINE]; SCANLINES],
) -> Vec<String> {
    buffer
        .iter()
        .enumerate()
        .map(|(sl, data)| {
            data.iter().fold(format!("{sl:03} => "), |acc, &e| {
                acc + ((if e == 0 {
                    "│  ".to_string()
                } else {
                    format!("│{e:02X}")
                })
                .as_str())
            })
        })
        .collect()
}

pub fn read_rom(relative_path: &str) -> Vec<u8> {
    let bin_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "bins", relative_path]
        .iter()
        .collect();

    fs::read(bin_path).unwrap()
}
