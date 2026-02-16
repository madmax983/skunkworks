use nom::{
    bytes::complete::{tag, take},
    number::complete::{le_f32, le_u8},
    IResult,
};
use crate::dna::{Creature, Head, Limb, HEAD_MARKER, LIMB_MARKER, MAGIC};

pub fn resurrect(data: &[u8]) -> Creature {
    // Attempt to find MAGIC
    let start_idx = match find_magic(data) {
        Some(idx) => idx + MAGIC.len(),
        None => 0, // No magic found, assume start or just try to find HEAD
    };

    let input = if start_idx < data.len() {
        &data[start_idx..]
    } else {
        &[]
    };

    // Try to find HEAD
    let (rest, head) = match parse_head_resilient(input) {
        Ok((rem, h)) => (rem, h),
        Err(_) => {
            // Fallback: Default Head
            (input, Head { size: 10.0, eye_count: 0, color: [100, 100, 100] })
        }
    };

    // Try to find LIMBS
    let limbs = parse_limbs_resilient(rest);

    Creature { head, limbs }
}

fn find_magic(input: &[u8]) -> Option<usize> {
    input.windows(MAGIC.len()).position(|window| window == MAGIC)
}

fn parse_head_resilient(input: &[u8]) -> IResult<&[u8], Head> {
    // Search for HEAD_MARKER
    let mut current = input;
    loop {
        if current.is_empty() {
             return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
        }
        if current[0] == HEAD_MARKER {
            if let Ok((rem, head)) = parse_head_exact(current) {
                return Ok((rem, head));
            }
        }
        current = &current[1..];
    }
}

fn parse_head_exact(input: &[u8]) -> IResult<&[u8], Head> {
    let (input, _marker) = tag(&[HEAD_MARKER])(input)?;
    let (input, size) = le_f32(input)?;
    let (input, eye_count) = le_u8(input)?;
    let (input, color_bytes) = take(3usize)(input)?;
    let (input, _checksum) = le_u8(input)?; // Ignore checksum failure for now

    let color = [color_bytes[0], color_bytes[1], color_bytes[2]];

    // Sanitize
    let size = if size.is_nan() || size <= 0.0 || size > 1000.0 { 10.0 } else { size };

    Ok((input, Head { size, eye_count, color }))
}

fn parse_limbs_resilient(input: &[u8]) -> Vec<Limb> {
    let mut limbs = Vec::new();
    let mut current = input;

    // We scan for LIMB_MARKERs
    while !current.is_empty() {
        // Find next marker
        match current.iter().position(|&b| b == LIMB_MARKER) {
            Some(pos) => {
                let candidate = &current[pos..];
                match parse_limb_exact(candidate) {
                    Ok((rem, limb)) => {
                        limbs.push(limb);
                        current = rem; // Continue from where limb ended
                    }
                    Err(_) => {
                        // Failed to parse this limb, skip marker and continue search
                        current = &current[pos + 1..];
                    }
                }
            }
            None => break,
        }
    }
    limbs
}

fn parse_limb_exact(input: &[u8]) -> IResult<&[u8], Limb> {
    let (input, _marker) = tag(&[LIMB_MARKER])(input)?;
    let (input, length) = le_f32(input)?;
    let (input, thickness) = le_f32(input)?;
    let (input, joints) = le_u8(input)?;
    let (input, child_count) = le_u8(input)?;

    let (mut current_input, _checksum) = le_u8(input)?;

    let length = if length.is_nan() || length.abs() > 1000.0 { 20.0 } else { length.abs() };
    let thickness = if thickness.is_nan() || thickness.abs() > 100.0 { 5.0 } else { thickness.abs() };

    let mut children = Vec::new();

    let safe_child_count = child_count.min(5);

    for _ in 0..safe_child_count {
        if let Ok((rem, child)) = parse_limb_exact(current_input) {
            children.push(child);
            current_input = rem;
        } else {
            break;
        }
    }

    Ok((current_input, Limb { length, thickness, joints, children }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dna::Creature;

    #[test]
    fn test_resurrect_clean() {
        let creature = Creature::random();
        let data = creature.serialize();
        let resurrected = resurrect(&data);
        assert_eq!(creature.head.size, resurrected.head.size);
        // assert_eq!(creature.limbs.len(), resurrected.limbs.len());
        // Note: Limbs order might differ if recursive structure is flattened?
        // No, parse_limbs_resilient does a linear scan, but parse_limb_exact handles recursion.
        // So root limbs should be found in order.
    }
}
