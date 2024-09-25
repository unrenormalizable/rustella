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
