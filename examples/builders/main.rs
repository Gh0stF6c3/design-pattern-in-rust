
// 模式类型: 创建型
// 设计模式: 建造者
// 适用场景: 假设有一个复杂对象，创建这个对象需要对诸多成员变量和嵌套对象进行频繁的初始化工作。
//           而这些初始化代码通常藏在众多参数和让人看不懂的构造函数中，或者这些参数散落在客户端
//           （调用者）的各个位置。而构建者模式可以通过生成器以此逐步创建一个复杂对象。

// 这里注意区分模式`Builder`和`Fluent Interface`（即流式接口），虽然两者都是逐步构建对象，但是
// Builder允许使用相同的构建过程来构建出不同的产品

use builder::{car::{Builder, CarBuilder}, car_manual::CarManualBuilder};
use product::director::Director;

mod builder;
mod product;

fn main() {
    let mut car_builder = CarBuilder::default();
    Director::construct_suv(&mut car_builder);
    let _ = car_builder.build();

    let mut manual_builder = CarManualBuilder::default();
    Director::construct_city_car(&mut manual_builder);
    let car = manual_builder.build();

    println!("{}",car)
}