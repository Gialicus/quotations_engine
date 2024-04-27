use nanoid::nanoid;

#[derive(Debug, Clone)]
pub struct Price {
    pub id: String,
    pub product_id: String,
    pub price: f64,
    pub discount: f64,
    pub lead_time: u32,
}
impl Price {
    pub fn new(product_id: &str, price: f64, discount: f64, lead_time: u32) -> Self {
        Self {
            id: nanoid!(),
            product_id: product_id.into(),
            price,
            discount,
            lead_time,
        }
    }
}
