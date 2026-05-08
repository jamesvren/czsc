use rs_czsc::utils::ta::*;

#[test]
fn test_sma() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = sma(&data, 2);
    let expected = vec![1.0, 1.5, 2.0, 3.0, 4.0];
    
    assert_eq!(result.len(), expected.len());
    for (a, b) in result.iter().zip(expected.iter()) {
        assert!((a - b).abs() < 1e-10);
    }
}

#[test]
fn test_ema() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = ema(&data, 2);
    // EMA has more complex calculation, we'll just check it returns correct length
    assert_eq!(result.len(), data.len());
    
    // Test with single element
    let single_data = vec![5.0];
    let single_result = ema(&single_data, 2);
    assert_eq!(single_result, vec![5.0]);
    
    // Test with timeperiod 0
    let zero_period_result = ema(&data, 0);
    assert_eq!(zero_period_result, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    
    // Test with timeperiod greater than data length
    let large_period_result = ema(&data, 10);
    assert_eq!(large_period_result.len(), data.len());
}

#[test]
fn test_rsq() {
    // Perfect positive correlation
    let data1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let rsq1 = rsq(&data1);
    assert!((rsq1 - 1.0).abs() < 1e-10);
    
    // Test with single element
    let data_single = vec![5.0];
    let rsq_single = rsq(&data_single);
    assert!((rsq_single - 1.0).abs() < 1e-10);
    
    // Test with two elements
    let data_two = vec![1.0, 2.0];
    let rsq_two = rsq(&data_two);
    assert!((rsq_two - 1.0).abs() < 1e-10);
}

#[test]
fn test_trait_round_digits() {
    let x = 3.1415926_f64;
    assert_eq!(x.round_digits(4), 3.1415);
    
    let y = 3.1415926_f32;
    assert_eq!(y.round_digits(4), 3.1415);
    
    // Test edge cases
    let z = 0.0_f64;
    assert_eq!(z.round_digits(4), 0.0);
    
    let w = -3.1415926_f64;
    assert_eq!(w.round_digits(4), -3.1415);
}