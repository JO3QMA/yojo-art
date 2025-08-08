use pad::{Alignment, PadStr};

/// ゼロ埋め（`pad`）に使うトレイトと、埋め方向（`Alignment`）を提供するクレート。
/// `"abc".pad(8, '0', Alignment::Right, false)` のように使い、右側に詰めて左側を`0`で埋める。

use super::IdServiceImpl;

/// 16進数の許可文字。ランダム部分の生成で使用する。
const CHARS: &'static str = "0123456789abcdef";

// ObjectIdはMongoDBのObjectIdと同じ形式の「24桁の16進文字列」を採用します。
// 厳密なMongoDBの構成（4B=秒, 3B=マシンID, 2B=PID, 3B=カウンタ）までは再現せず、
// この実装では先頭8桁（=4B）にUNIX秒を16進で入れ、残り16桁は16進のランダム文字で埋めます。
// そのため、後半からマシンIDやPIDを復元することは想定しません（単純・高速・十分な一意性が目的）。

#[derive(Debug)]
pub struct ObjectIdService;
impl IdServiceImpl for ObjectIdService {
    /// 時刻`t`が安全な値かを判定します。
    /// ここでは 0 より大きい（= UNIXエポック以降）を安全とみなします。
	fn is_safe_t(&self, t: i64) -> bool {
		t > 0
	}
    /// 与えられたミリ秒時刻`time`からIDを生成します。
    /// 先頭8桁は `floor(time_ms/1000)` を16進でゼロ埋め、続く16桁は16進のランダム文字です。
    fn gen_id(&self, time: i64) -> String {
        // ランダム16桁（[0-9a-f]）を生成
        let random = nanoid::nanoid!(16, &CHARS.chars().collect::<Vec<char>>());
        // 8桁の秒（16進, 左ゼロ埋め）+ ランダム16桁 で24桁になる
        get_time(time) + &random
    }
    /// 生成済みIDから時刻を復元します。
    /// 先頭8桁を16進の秒として読み取り、ミリ秒（×1000）に変換して返します。
    /// 失敗時は`None`を返します（`ok()?`はエラーなら早期に`None`へ変換）。
    fn parse(&self, id: &str) -> Option<i64> {
        Some(i64::from_str_radix(&id[0..8], 16).ok()? * 1000)
    }
}
impl ObjectIdService {
    /// サービスの新規作成。特別な状態は持たないため、毎回同じ構造体を返します。
	pub fn new() -> Self {
		Self
	}
}

/// ミリ秒の時刻を「16進の秒（8桁・左ゼロ埋め）」に変換します。
/// 注意: 現状 `time == 0` のとき "0" を返すため、8桁ではなく1桁になります。
///       固定長を期待する場合は仕様の見直しが必要です（TODO）。
fn get_time(time: i64) -> String {
    // 0未満は0に切り上げ（負の時間は扱わない）
    let time = time.max(0);
    if time == 0 {
        // 現実装では特例で1文字"0"を返す。固定長8桁にするかは要件次第。
        return CHARS[0..1].to_string();
    }

    // ミリ秒→秒へ（小数は切り捨て）
    let time = (time as f64 / 1000.0).floor() as i64;

    // BigIntにしてから16進文字列に変換し、左側を'0'で埋めて8桁にする
    use num::FromPrimitive;
    num::BigInt::from_i64(time)
        .unwrap()
        .to_str_radix(16)
        .pad(8, '0', Alignment::Right, false)
}

#[cfg(test)]
mod tests;
