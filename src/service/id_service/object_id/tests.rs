use super::*;

fn is_hex(s: &str) -> bool {
	s.chars().all(|c| c.is_ascii_hexdigit())
}

#[test]
fn gen_id_format_and_length() {
	let svc = ObjectIdService::new();
	let now = 1_700_000_000_000i64;
	let id = svc.gen_id(now);
	assert_eq!(id.len(), 24, "expected 24 chars");
	assert!(is_hex(&id), "id must be hex");
	// 先頭8桁は秒（16進）
	let secs_hex = (now as f64 / 1000.0).floor() as i64;
	let secs_hex = format!("{:x}", secs_hex);
	assert!(id.starts_with(&secs_hex[secs_hex.len().saturating_sub(8)..]));
}

#[test]
fn parse_round_trip_with_floor_ms() {
	let svc = ObjectIdService::new();
	let t = 1_700_000_000_123i64; // 123ms 付き
	let id = svc.gen_id(t);
	let parsed = svc.parse(&id).expect("parse should succeed");
	let expected = ((t as f64) / 1000.0).floor() as i64 * 1000;
	assert_eq!(parsed, expected);
}

#[test]
#[ignore]
fn boundary_time_zero_behavior_needs_spec() {
	// 現状 get_time(0) は "0" を返すため固定長と不整合の恐れがある。
	// 仕様決定後、このテストを更新/有効化する。
	let svc = ObjectIdService::new();
	let id = svc.gen_id(0);
	// 少なくとも16進文字であることは維持されるべき
	assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
}
