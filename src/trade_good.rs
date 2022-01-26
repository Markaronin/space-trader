use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TradeGood {
    Iron,
    Wood,
    Gold,
}
impl TradeGood {
    pub fn default_prices() -> BTreeMap<TradeGood, usize> {
        let mut prices = BTreeMap::new();
        prices.insert(Self::Iron, 20);
        prices.insert(Self::Wood, 10);
        prices.insert(Self::Gold, 100);
        prices
    }
}
