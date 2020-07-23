use uuid::Uuid;

// The bootstrap registry.

#[macro_export]
macro_rules! uuid {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {{
        use uuid::Uuid;

        let e: u64 = $e;
        Uuid::from_bytes([
            (($a >> 24) & 0xff) as u8,
            (($a >> 16) & 0xff) as u8,
            (($a >> 8) & 0xff) as u8,
            ($a & 0xff) as u8,
            (($b >> 8) & 0xff) as u8,
            ($b & 0xff) as u8,
            (($c >> 8) & 0xff) as u8,
            ($c & 0xff) as u8,
            (($d >> 8) & 0xff) as u8,
            ($d & 0xff) as u8,
            ((e >> 40) & 0xff) as u8,
            ((e >> 32) & 0xff) as u8,
            ((e >> 24) & 0xff) as u8,
            ((e >> 16) & 0xff) as u8,
            ((e >> 8) & 0xff) as u8,
            (e & 0xff) as u8,
        ])
    }};
}

#[macro_export]
macro_rules! format_uuid {
    ($a:expr) => {{
        let (a, b, c, r) = $a.as_fields();
        let d: u16 = ((r[0] as u16) << 8) + (r[1] as u16);
        // let e = vec![r[2], r[3], r[4], r[5], r[6]];
        format!("uuid![0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x]", a, b, c, d)
    }};
}

pub fn get(key: &str) -> Uuid {
    Uuid::new_v5(&ROOT, key.as_bytes())
}

// const ROOT: Uuid = Uuid::from_bytes([
//     59, 61, 190, 239, 163, 188, 64, 80, 180, 51, 46, 6, 58, 57, 12, 149,
// ]);

pub const ROOT: Uuid = uuid![0x3b3dbeef, 0xa3bc, 0x4050, 0xb433, 0x2e063a390c95];
pub const NAME: Uuid = uuid![0x3817b373, 0x9a8d, 0x423d, 0x9f0d, 0xcd94e9a714ed];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_macro_test() {
        assert_eq!(
            ROOT,
            Uuid::parse_str("3b3dbeef-a3bc-4050-b433-2e063a390c95").unwrap()
        )
    }
}
