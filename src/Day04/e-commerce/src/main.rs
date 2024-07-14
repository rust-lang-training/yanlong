// 题目：https://blog-huahua.oss-cn-beijing.aliyuncs.com/blog/code/day4_homework.png
// 效果：https://blog-huahua.oss-cn-beijing.aliyuncs.com/blog/code/market.gif

mod order;
use order::Order;
mod product;
use product::Product;
use product::Category;
mod utils;
use utils::*;


fn main() {
    // 余额
    let mut remain:f32 = 1000.0;
    // 订单列表，支付成功的时候 清除订单
    let mut order_list: Vec<Order> = Vec::new();
    // 产品列表，库存随着下单而减少
    let mut product_list = vec![
        Product { _id: 1,  name: "watermelon".to_string(), category: Category::Fruit, stock: 20, price: 20.0 },
        Product { _id: 2,  name: "orange".to_string(), category: Category::Fruit, stock: 20, price: 10.0 },
        Product { _id: 3,  name: "pumpkin".to_string(), category: Category::Vegetable, stock: 110, price: 4.0 },
        Product { _id: 4,  name: "corn".to_string(), category: Category::Vegetable, stock: 50, price: 10.0 },
        Product { _id: 5,  name: "chicken".to_string(), category: Category::Meat, stock: 20, price: 30.0 },
        Product { _id: 6,  name: "pork".to_string(), category: Category::Meat, stock: 40, price: 35.0 },
    ];

    loop {
        print_main_menu();
        // 获取菜单，
        let menu = get_stdin("请选择菜单: ");
        // 各菜单处理
        match &menu[0..] {
            "0" => break,
            "1" => browse_products_menu(&product_list),
            "2" => search_product_menu(&product_list),
            "3" => order_product_menu(&mut product_list, &mut order_list),
            "4" => compute_order_menu(&order_list),
            "5" => pay_order_menu(&mut remain, &mut order_list),
            _ => println!("请输入0-5的整数"),
        }

    }
}