pub fn parse_u16_lines(s: &str) -> Vec<u16> {
    let mut v = Vec::with_capacity(512);

    let mut current = 0;
    let mut remaining = false;
    for byte in s.bytes() {
        match byte.checked_sub(48) {
            Some(b) if b < 10 => {
                remaining = true;
                current *= 10;
                current += b as u16;
            }
            _ => {
                if remaining {
                    remaining = false;
                    v.push(current);
                    current = 0;
                }
            }
        };
    }

    if remaining {
        v.push(current);
    }

    v
}
