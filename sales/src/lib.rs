#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Self {
        Self { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Self {
        Self { items: vec![], receipt: vec![] }
    }

    pub fn insert_item(&mut self, s: &Store, name: String) {
        if let Some((_, price)) = s.products.iter().find(|(n, _)| *n == name) {
            self.items.push((name, *price));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let free_count = self.items.len() / 3;
        let paid: Vec<f32> = prices[free_count..].to_vec();
        let total_paid: f32 = paid.iter().sum();
        let total_original: f32 = prices.iter().sum();
        let ratio = total_paid / total_original;

        self.receipt = prices.iter().map(|p| round_two(p * ratio)).collect();
        self.receipt.clone()
    }
}

fn round_two(n: f32) -> f32 {
    (n * 100.0).round() / 100.0
}
