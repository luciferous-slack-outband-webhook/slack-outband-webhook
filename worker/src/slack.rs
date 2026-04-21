use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub const TIMESTAMP_TOLERANCE_SECONDS: u64 = 5 * 60;

#[derive(Debug, PartialEq)]
pub enum VerifyError {
    TimestampParseError,
    TimestampTooOld,
    InvalidSignatureFormat,
    InvalidHex,
    SignatureMismatch,
    HmacInitError,
}

pub fn verify_slack_signature(
    timestamp: &str,
    signature: &str,
    body: &str,
    signing_secret: &[u8],
    now_seconds: u64,
) -> Result<(), VerifyError> {
    let ts: u64 = timestamp
        .parse()
        .map_err(|_| VerifyError::TimestampParseError)?;

    if now_seconds.abs_diff(ts) > TIMESTAMP_TOLERANCE_SECONDS {
        return Err(VerifyError::TimestampTooOld);
    }

    let expected_signature = signature
        .strip_prefix("v0=")
        .ok_or(VerifyError::InvalidSignatureFormat)?;
    let expected_bytes = hex::decode(expected_signature).map_err(|_| VerifyError::InvalidHex)?;

    let sig_basestring = format!("v0:{timestamp}:{body}");
    let mut mac =
        HmacSha256::new_from_slice(signing_secret).map_err(|_| VerifyError::HmacInitError)?;
    mac.update(sig_basestring.as_bytes());
    mac.verify_slice(&expected_bytes)
        .map_err(|_| VerifyError::SignatureMismatch)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Slack公式ドキュメントのexample value
    const SECRET: &[u8] = b"8f742231b10e8888abcd99yyyzzz85a5";
    const TS: &str = "1531420618";
    const BODY: &str = "token=xyzz0WbapA4vBCDEFasx0q6G&team_id=T1DC2JH3J&team_domain=exampleteam&\
         channel_id=C2147483705&channel_name=test&user_id=U2147483697&user_name=Steve&\
         text=googlebot%3A+What+is+the+air-speed+velocity+of+an+unladen+swallow%3F\
         &response_url=https%3A%2F%2Fhooks.slack.com%2Factions%2FT1DC2JH3J%2F397700885554\
         %2F96277235169c3d85e6d0be9e0b3f8a2b&trigger_id=398738468139.47445629121.803a0bc887a\
         14d8e30";

    fn compute_sig(secret: &[u8], ts: &str, body: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(secret).unwrap();
        mac.update(format!("v0:{ts}:{body}").as_bytes());
        format!("v0={}", hex::encode(mac.finalize().into_bytes()))
    }

    fn ts() -> u64 {
        TS.parse().unwrap()
    }

    #[test]
    fn accepts_valid_signature() {
        let sig = compute_sig(SECRET, TS, BODY);
        assert_eq!(verify_slack_signature(TS, &sig, BODY, SECRET, ts()), Ok(()));
    }

    #[test]
    fn rejects_tampered_body() {
        let sig = compute_sig(SECRET, TS, BODY);
        assert_eq!(
            verify_slack_signature(TS, &sig, "tampered", SECRET, ts()),
            Err(VerifyError::SignatureMismatch)
        );
    }

    #[test]
    fn rejects_timestamp_too_old() {
        let sig = compute_sig(SECRET, TS, BODY);
        let now = ts() + 301;
        assert_eq!(
            verify_slack_signature(TS, &sig, BODY, SECRET, now),
            Err(VerifyError::TimestampTooOld)
        );
    }

    #[test]
    fn accepts_timestamp_within_tolerance() {
        let sig = compute_sig(SECRET, TS, BODY);
        let now = ts() + 299;
        assert_eq!(verify_slack_signature(TS, &sig, BODY, SECRET, now), Ok(()));
    }

    #[test]
    fn rejects_future_timestamp() {
        // abs_diff を使っているため未来方向も弾く
        let sig = compute_sig(SECRET, TS, BODY);
        let now = ts() - 301;
        assert_eq!(
            verify_slack_signature(TS, &sig, BODY, SECRET, now),
            Err(VerifyError::TimestampTooOld)
        );
    }

    #[test]
    fn rejects_missing_v0_prefix() {
        let sig = "invalidsignature";
        assert_eq!(
            verify_slack_signature(TS, sig, BODY, SECRET, ts()),
            Err(VerifyError::InvalidSignatureFormat)
        );
    }

    #[test]
    fn rejects_non_hex_signature() {
        let sig = "v0=notahex!!";
        assert_eq!(
            verify_slack_signature(TS, sig, BODY, SECRET, ts()),
            Err(VerifyError::InvalidHex)
        );
    }

    #[test]
    fn rejects_invalid_timestamp() {
        let sig = compute_sig(SECRET, TS, BODY);
        assert_eq!(
            verify_slack_signature("abc", &sig, BODY, SECRET, ts()),
            Err(VerifyError::TimestampParseError)
        );
    }

    #[test]
    fn accepts_empty_body() {
        let sig = compute_sig(SECRET, TS, "");
        assert_eq!(verify_slack_signature(TS, &sig, "", SECRET, ts()), Ok(()));
    }

    #[test]
    fn accepts_empty_signing_secret() {
        // new_from_slice は空キーでも成功する（HmacInitError には到達しない）
        let sig = compute_sig(b"", TS, BODY);
        assert_eq!(verify_slack_signature(TS, &sig, BODY, b"", ts()), Ok(()));
    }
}
