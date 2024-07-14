use std::fmt;
/*
 *  Order相关结构体和方法定义
 */
#[derive(Debug)]
pub struct Order {
    pub name: String,
    pub price: f32,
    pub buy_count: u32,
}
impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Order { name, price, buy_count } = self;
        // f是固定参数，{:<10}左对齐，长度为10
        write!(f, "{:<20} {:<20} {:<20} ", name, price, buy_count)
    }
} 