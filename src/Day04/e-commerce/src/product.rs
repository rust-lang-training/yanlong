use std::fmt;

/*
 *  Product相关结构体和方法定义
 */
#[derive(Debug, Clone)] // Debug 默认实现了 Display trait

pub struct Product {
    pub _id: u32,
    pub name: String,
    pub category: Category,
    pub stock: u32,
    pub price: f32,
}


impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Product { name, category, stock, price, .. } = self;
        let category = match category {
            Category::Fruit => "Fruit",
            Category::Vegetable => "Vegetable",
            Category::Meat => "Meat",
        };
        // f是固定参数，{:<10}左对齐，长度为10
        write!(f, "{:<20} {:<20} {:<20} {:<20}", name, category, stock, price)
    }
} 

/**
 * Category的枚举
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Category {
    Fruit,
    Vegetable,
    Meat,
}