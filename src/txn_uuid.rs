/*
 * PTA-Generator 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::setup::SetSize;
use jiff::Timestamp;
use uuid::Uuid;

/// Generate stable, predictable UUID for test data generation
///
/// Some integration tests need a stable set of unique txn UUIDs
/// This function will generate these kind of UUIDs.
/// In real life use, just normal random UUID is suitable for txn uuid.
///
/// This is V5 UUID with URL namespace and the used url is:
/// `pta-generator:/<set>/<number>/<ts>[/<spec>]`
/// where `ts` is timestamp in RFC 3339 format,
/// and `spec` is free, optional specifier, e.g. `audit`, or `plain`.
pub fn get_txn_uuid(set: &SetSize, number: u32, ts: Timestamp, spec: Option<&str>) -> String {
    let data = format!(
        "pta-generator:/{}/{}/{}{}",
        set.str(),
        number,
        ts,
        spec.map_or(String::default(), |s| format!("/{}", s))
    );
    Uuid::new_v5(&Uuid::NAMESPACE_URL, data.as_bytes()).to_string()
}

#[cfg(test)]
mod tests {
    use crate::setup::SetSize;
    use crate::txn_uuid::get_txn_uuid;
    use uuid::Uuid;

    #[test]
    fn test_v5_uuid() {
        assert_eq!(
            "94942e79-0153-53d1-9b12-a13f788cbaf1",
            Uuid::new_v5(
                &Uuid::NAMESPACE_URL,
                b"pta-generator:/1e3/1/2024-01-02T00:01:02Z"
            )
            .to_string()
        );

        assert_eq!(
            "eed4b0e3-cd5e-5a31-91ee-31c2e65ddb12",
            Uuid::new_v5(
                &Uuid::NAMESPACE_URL,
                b"pta-generator:/1e3/999/2024-12-31T23:58:00Z"
            )
            .to_string()
        );

        assert_eq!(
            "4a039481-bf0d-5428-8771-d1a771649037",
            Uuid::new_v5(
                &Uuid::NAMESPACE_URL,
                b"pta-generator:/1e6/1/2024-01-02T00:01:02Z"
            )
            .to_string()
        );

        assert_eq!(
            "8e43c795-8fb1-552e-9dde-eae36f233676",
            Uuid::new_v5(
                &Uuid::NAMESPACE_URL,
                b"pta-generator:/1e3/999/2024-12-31T23:58:00Z/audit"
            )
            .to_string()
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_txn_uuid() {
        let tests = vec![
            ("94942e79-0153-53d1-9b12-a13f788cbaf1", SetSize::Sz1e3, 1, "2024-01-02T00:01:02Z", None),
            ("eed4b0e3-cd5e-5a31-91ee-31c2e65ddb12", SetSize::Sz1e3, 999, "2024-12-31T23:58:00Z", None),
            ("4a039481-bf0d-5428-8771-d1a771649037", SetSize::Sz1e6, 1, "2024-01-02T00:01:02Z", None),
            ("5e2bae5f-b79c-5515-a617-e53b955209b3", SetSize::Sz1e6, 999, "2024-12-31T23:58:00Z", None),
            ("e36e59d4-6e1d-5685-8c53-7f7ef8254861", SetSize::Sz1e3, 1, "2024-01-02T00:01:02Z", Some("plain")),
            ("8e43c795-8fb1-552e-9dde-eae36f233676", SetSize::Sz1e3, 999, "2024-12-31T23:58:00Z", Some("audit")),
        ];

        for test in tests {
            let uuid = get_txn_uuid(&test.1, test.2, test.3.parse().unwrap(/*:test:*/), test.4);
            assert_eq!(test.0, &uuid);
        }
    }
}
