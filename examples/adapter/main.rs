
// 模式类型: 结构型
// 设计模式: 设配器
// 适用场景: 适配器模式是一种结构型设计模式， 它能使接口不兼容的对象能够相互合作。例如项目只能接收XML文件。
//          此时如果调用了一个只能接收JSON文件作为数据分析来源的第三方库，那么就需要考虑到兼容性问题。可以
//          通过创建一个适配器转换接口对象，从而使其能够与其他对象交互。

use adaptee::SpecificTarget;
use adapter::TargetAdapter;
use target::OrdinaryTarget;

mod adaptee;
mod adapter;
mod target;

fn main() {
    let target = OrdinaryTarget;
    target::call(target);

    let spec_target = SpecificTarget;
    // 这里进行了对象转换处理
    target::call(
        TargetAdapter::new(spec_target)
    );
}