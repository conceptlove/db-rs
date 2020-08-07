use uuid::Uuid;

pub type Id = uuid::Uuid;

#[macro_export]
macro_rules! uuid {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {{
        use uuid::Uuid;

        let e: u64 = $e;
        let a: u32 = $a;

        Uuid::from_bytes([
            ((a >> 24) & 0xff) as u8,
            ((a >> 16) & 0xff) as u8,
            ((a >> 8) & 0xff) as u8,
            (a & 0xff) as u8,
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

pub fn id(st: &String) -> Id {
    Id::parse_str(st).unwrap()
}

pub fn get(key: &str) -> Uuid {
    Uuid::new_v5(&CORE, key.as_bytes())
}

pub fn new() -> Uuid {
    Uuid::new_v4()
}

pub const CORE: Uuid = uuid![0x3b3dbeef, 0xa3bc, 0x4050, 0xb433, 0x2e063a390c95];

pub const FIRST: Uuid = uuid![0x00000000, 0x0000, 0x0000, 0x0000, 0x000000000000];
pub const LAST: Uuid = uuid![0xffffffff, 0xffff, 0xffff, 0xffff, 0xffffffffffff];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_macro_test() {
        assert_eq!(
            CORE,
            Uuid::parse_str("3b3dbeef-a3bc-4050-b433-2e063a390c95").unwrap()
        )
    }
}
