use core::fmt::Debug;

pub trait TV: Debug {
    fn render(&mut self, scan_line: usize, offset: usize, data: u8);
}

#[derive(Debug)]
pub struct InMemoryTV<const MAX_SCAN_LINES: usize, const PIXELS_PER_SCAN_LINE: usize> {
    buffer: [[u8; PIXELS_PER_SCAN_LINE]; MAX_SCAN_LINES],
}

impl<const MAX_SCAN_LINES: usize, const PIXELS_PER_SCAN_LINE: usize> Default
    for InMemoryTV<MAX_SCAN_LINES, PIXELS_PER_SCAN_LINE>
{
    fn default() -> Self {
        Self {
            buffer: [[0x00; PIXELS_PER_SCAN_LINE]; MAX_SCAN_LINES],
        }
    }
}

impl<const MAX_SCAN_LINES: usize, const PIXELS_PER_SCAN_LINE: usize>
    InMemoryTV<MAX_SCAN_LINES, PIXELS_PER_SCAN_LINE>
{
    pub fn buffer(&self) -> &[[u8; PIXELS_PER_SCAN_LINE]; MAX_SCAN_LINES] {
        &self.buffer
    }
}

impl<const MAX_SCAN_LINES: usize, const PIXELS_PER_SCAN_LINE: usize> TV
    for InMemoryTV<MAX_SCAN_LINES, PIXELS_PER_SCAN_LINE>
{
    #[inline]
    fn render(&mut self, scan_line: usize, offset: usize, data: u8) {
        if scan_line >= MAX_SCAN_LINES || offset >= PIXELS_PER_SCAN_LINE {
            return;
        }

        self.buffer[scan_line][offset] = data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const MAX_SCAN_LINES: usize = 1000;
    const PIXELS_PER_SCAN_LINE: usize = 1000;

    #[test_case(0, 0)]
    #[test_case(MAX_SCAN_LINES, 10; "Overflow past last line")]
    #[test_case(10, PIXELS_PER_SCAN_LINE; "Overflow past last pixed")]
    #[test_case(MAX_SCAN_LINES, PIXELS_PER_SCAN_LINE; "Overflow past last line & pixel")]
    fn render_test(scan_line: usize, offset: usize) {
        let mut tv = InMemoryTV::<MAX_SCAN_LINES, PIXELS_PER_SCAN_LINE>::default();

        tv.render(scan_line, offset, 0xFF);

        let cheker = |sl: usize, off: usize, val: u8| -> bool {
            if sl == scan_line && off == offset {
                val == 0xFF
            } else {
                val == 0x00
            }
        };

        assert!(tv.buffer().iter().enumerate().all(|(sl, &buf)| buf
            .iter()
            .enumerate()
            .all(|(off, &val)| cheker(sl, off, val))))
    }
}
