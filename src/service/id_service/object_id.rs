use pad::{Alignment, PadStr};

use super::IdServiceImpl;

const CHARS: &'static str = "0123456789abcdef";

#[derive(Debug)]
pub struct ObjectIdService;
impl IdServiceImpl for ObjectIdService {
    fn is_safe_t(&self, t: i64) -> bool {
        t > 0
    }
    fn gen_id(&self, time: i64) -> String {
        let random = nanoid::nanoid!(16, &CHARS.chars().collect::<Vec<char>>());
        get_time(time) + &random
    }
    fn parse(&self, id: &str) -> Option<i64> {
        Some(i64::from_str_radix(&id[0..8], 16).ok()? * 1000)
    }
}
impl ObjectIdService {
    pub fn new() -> Self {
        Self
    }
}

fn get_time(time: i64) -> String {
    let time = time.max(0);
    if time == 0 {
        return CHARS[0..1].to_string();
    }

    let time = (time as f64 / 1000.0).floor() as i64;

    use num::FromPrimitive;
    num::BigInt::from_i64(time)
        .unwrap()
        .to_str_radix(16)
        .pad(8, '0', Alignment::Right, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_t() {
        let service = ObjectIdService::new();
        assert!(service.is_safe_t(1000));
        assert!(!service.is_safe_t(0));
        assert!(!service.is_safe_t(-1000));
    }

    #[test]
    fn test_gen_id() {
        let service = ObjectIdService::new();
        let id = service.gen_id(1000);
        assert_eq!(id.len(), 24); // 8 chars for time + 16 chars for random
        assert!(id.chars().all(|c| CHARS.contains(c)));
    }

    #[test]
    fn test_parse() {
        let service = ObjectIdService::new();
        let time = 1632482766000; // Example timestamp
        let id = service.gen_id(time);
        let parsed = service.parse(&id);
        assert_eq!(parsed, Some(time));
    }

    #[test]
    fn test_get_time() {
        assert_eq!(get_time(0), "0");
        assert_eq!(get_time(-1000), "0");
        
        // Test positive timestamps
        assert_eq!(get_time(1000), "000003e8");
        assert_eq!(get_time(1632482766000), "6149d96e");
        
        // Test padding
        assert_eq!(get_time(1000).len(), 8);
        assert_eq!(get_time(1632482766000).len(), 8);
    }
}

    #[test]
    fn test_is_safe_t() {
        let service = ObjectIdService::new();
        assert!(service.is_safe_t(1000));
        assert!(!service.is_safe_t(0));
        assert!(!service.is_safe_t(-1000));
    }

    #[test]
    fn test_gen_id() {
        let service = ObjectIdService::new();
        let id = service.gen_id(1000);
        assert_eq!(id.len(), 24); // 8 chars for time + 16 chars for random
        assert!(id.chars().all(|c| CHARS.contains(c)));
    }

    #[test]
    fn test_parse() {
        let service = ObjectIdService::new();
        let time = 1632482766000; // Example timestamp
        let id = service.gen_id(time);
        let parsed = service.parse(&id);
        assert_eq!(parsed, Some(time));
    }

    #[test]
    fn test_get_time() {
        assert_eq!(get_time(0), "0");
        assert_eq!(get_time(-1000), "0");
        
        // Test positive timestamps
        assert_eq!(get_time(1000), "000003e8");
        assert_eq!(get_time(1632482766000), "6149d96e");
        
        // Test padding
        assert_eq!(get_time(1000).len(), 8);
        assert_eq!(get_time(1632482766000).len(), 8);
    }
}
