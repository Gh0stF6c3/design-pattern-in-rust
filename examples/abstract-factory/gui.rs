//! 实现证明抽象工厂模式，这里假定gui定义了所有GUI组件需要实现的接口，`windows_gui`和`macos_gui`分别
//! 为其对应平台的接口实现

pub trait Button {
    fn press(&self);
}

pub trait CheckBox {
    fn switch(&self);
}

/// 这里分为两种分派调用方式： 静态（泛型）和动态（Box dyn），一般来说如果在库中则使用静态定义方式，而在
/// 二进制中调度最好使用动态Box dyn的方式进行定义、调度


// 使用泛型的抽象工厂
pub trait GuiFactory {
    type B: Button;
    type C: CheckBox;

    fn create_button(&self) -> Self::B;
    fn create_checkbox(&self) -> Self::C;
}

// 使用Box动态指针的抽象工厂
pub trait GuiFactoryDynamic {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn CheckBox>;
}