use rs_czsc::utils::*;

#[test]
fn test_x_round() {
    assert_eq!(x_round(3.1415926, 4), 3.1415);
    assert_eq!(x_round(-3.1415926, 4), -3.1415);
    assert_eq!(x_round(0.0, 4), 0.0);
    assert_eq!(x_round(1.0, 0), 1.0);
    assert_eq!(x_round(1.9, 0), 1.0);
}

#[test]
fn test_mac_address() {
    let mac = mac_address();
    // MAC address should be 17 characters in XX-XX-XX-XX-XX-XX format
    assert_eq!(mac.len(), 17);
    // Should contain only hex digits and dashes
    assert!(mac.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
}

#[test]
fn test_freqs_sorted() {
    let input = vec!["日线", "1分钟", "5分钟", "周线"];
    let sorted = freqs_sorted(input);
    let expected = vec!["1分钟", "5分钟", "日线", "周线"];
    
    assert_eq!(sorted, expected);
}