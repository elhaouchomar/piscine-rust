#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some((name, price)) = s.products.iter().find(|(n, _)| *n == ele) {
            self.items.push((name.clone(), *price));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
    let prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();
    let mut receipt: Vec<f32> = Vec::new();

    for chunk in prices.chunks(3) {
        let mut group: Vec<f32> = chunk.to_vec();
        if group.len() == 3 {
            let min_price = group.iter().cloned().fold(f32::INFINITY, f32::min);
            let sum: f32 = group.iter().sum();
            let new_sum = sum - min_price;
            let discount = new_sum / sum;
            group = group.iter().map(|p| (p * discount * 100.0).round() / 100.0).collect();

            let expected = (new_sum * 100.0).round() as i32;
            let actual: i32 = group.iter().map(|x| (x * 100.0).round() as i32).sum();
            let diff = expected - actual;
            if diff != 0 {
                if let Some(last) = group.last_mut() {
                    *last += diff as f32 / 100.0;
                }
            }
        }
        receipt.extend(group);
    }

    receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
    self.receipt = receipt.clone();
    receipt
}

}
