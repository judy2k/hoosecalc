pub fn pmt(r: f64, nper: i32, pv: f64) -> f64 {
    let payment_type = 0f64;
    let fv = 0.;
    if r == 0. {
        return -pv / nper as f64;
    }
    -r * (pv * (1. + r).powi(nper) + fv) / ((1. + r * payment_type) * ((1. + r).powi(nper) - 1.))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn test_pmt_values() {
        assert_ulps_eq!(pmt(0.025 / 12., 240, -360000.), 1907.650414915939);
        assert_ulps_eq!(pmt(0., 240, -360000.), 1500.);
        assert_ulps_eq!(pmt(0., 240, 360000.), -1500.);
        assert_ulps_eq!(pmt(0.025 / 12., 240, 360000.), -1907.650414915939);
        assert_ulps_eq!(pmt(0.025 / 12., 1, -360000.), 360749.99999998213);
        assert_ulps_eq!(pmt(0., 1, -360000.), 360000.);
    }
}
