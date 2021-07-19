use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::cmp::{max, min};
use std::collections::BTreeMap;

trait TaxCalculator {
    fn calculate(&self, offer_amount: Decimal) -> Decimal;
}

struct TieredCalculator {
    tiers: BTreeMap<Decimal, Decimal>,
}

impl TieredCalculator {
    fn new() -> Self {
        Self {
            tiers: BTreeMap::new(),
        }
    }

    fn threshold(&mut self, threshold: Decimal, multiplier: Decimal) {
        self.tiers.insert(threshold, multiplier);
    }
}

impl TaxCalculator for TieredCalculator {
    fn calculate(&self, offer_amount: Decimal) -> Decimal {
        let mut components = Vec::new();
        let mut tier_iter = self.tiers.iter().peekable();
        while let Some((lower_threshold, multiplier)) = tier_iter.next() {
            let upper_threshold = tier_iter.peek();
            let component = match upper_threshold {
                Some((upper_threshold, _)) => max(
                    min(&offer_amount, upper_threshold) - lower_threshold,
                    dec!(0),
                ),
                None => max(offer_amount - lower_threshold, dec!(0)),
            } * multiplier;
            components.push(component);
        }
        components.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    #[test]
    fn tier_calculation_real_values() {
        let mut calc = TieredCalculator::new();
        calc.threshold(dec!(0), dec!(0));
        calc.threshold(dec!(145000), dec!(0.02));
        calc.threshold(dec!(250000), dec!(0.05));
        calc.threshold(dec!(325000), dec!(0.1));
        calc.threshold(dec!(750000), dec!(0.12));

        assert_eq!(calc.calculate(dec!(0)), dec!(0));
        assert_eq!(calc.calculate(dec!(144999.99)), dec!(0));
        assert_eq!(calc.calculate(dec!(145000)), dec!(0));
        assert_eq!(calc.calculate(dec!(145001)), dec!(1) * dec!(0.02));
        assert_eq!(calc.calculate(dec!(250000)), dec!(105000) * dec!(0.02));
        assert_eq!(
            calc.calculate(dec!(250001)),
            dec!(105000) * dec!(0.02) + dec!(1) * dec!(0.05)
        );

        assert_eq!(calc.calculate(dec!(450000)), dec!(18350));
    }

    #[test]
    fn tier_calculation_empty_tiers() {
        assert_eq!(TieredCalculator::new().calculate(dec!(750000)), dec!(0));
        assert_eq!(TieredCalculator::new().calculate(dec!(0)), dec!(0));
        assert_eq!(TieredCalculator::new().calculate(dec!(-100000)), dec!(0))
    }
}
