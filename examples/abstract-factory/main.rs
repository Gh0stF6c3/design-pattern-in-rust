
// 模式类型: 创建型
// 设计模式: 抽象工厂
// 适用场景: 代码需要与多个不同系列的相关产品交互，但是无法提前获取相关消息，并且不希望
//           代码基于现有的基础产品进行构建，以保证具有一定的拓展性能。

//     示例: DEMO演示了一个GUI库是如何解决将同一函数接口拓展到不同平台的方法，该模式
//           允许让不同操作系统对接口进行代码实现，并且具有一定的拓展性以适配未来可能
//           存在的操作系统。


use gui::{Button, CheckBox, GuiFactory, GuiFactoryDynamic};
use macos_gui::{MacOSFactory, MacOSFactoryDyn};
use windows_gui::{WindowsFactory, WindowsFactoryDyn};

mod gui;
mod macos_gui;
mod windows_gui;

const WINDOWS_PLATFORM: bool = true;


// 这里不单独分离App逻辑，直接将渲染逻辑写入示例，函数实现如下：
fn render(factory: impl GuiFactory) {
    let button1 = factory.create_button();
    let checkbox = factory.create_checkbox();

    button1.press();
    checkbox.switch();
}

fn render_dyn(factory: &dyn GuiFactoryDynamic) {
    factory.create_button().press();
    factory.create_checkbox().switch();
}

fn main() {
    // 静态定义方式调用
    if WINDOWS_PLATFORM {
        render(WindowsFactory);
    }else {
        render(MacOSFactory);
    }

    // 动态定义方式调用
    let factory: &dyn GuiFactoryDynamic = if WINDOWS_PLATFORM {
        &WindowsFactoryDyn
    } else {
        &MacOSFactoryDyn
    };
    render_dyn(factory);
}