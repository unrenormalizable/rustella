#[cfg(target_arch = "x86_64")]
#[inline(never)]
pub fn get_nanoseconds() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(never)]
pub fn get_nanoseconds() -> u64 {
    0
}

pub fn measure_elapsed(start: u64) -> u64 {
    let end = get_nanoseconds();
    end.saturating_sub(start)
}

pub fn measure_overhead() -> u64 {
    let mut elapsed: u64 = 0;
    let count = 1_000;
    for _ in 1..count {
        let start = get_nanoseconds();
        elapsed += measure_elapsed(start);
    }

    elapsed / count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        assert!(measure_overhead() > 0)
    }
}
