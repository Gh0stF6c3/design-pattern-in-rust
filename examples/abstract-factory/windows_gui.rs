use crate::gui::{Button, CheckBox, GuiFactory, GuiFactoryDynamic};

pub struct WindowsFactory;
impl GuiFactory for WindowsFactory {
    type B = WindowsButton;
    type C = WindowsCheckBox;

    fn create_button(&self) -> Self::B {
        WindowsButton
    }

    fn create_checkbox(&self) -> Self::C {
        WindowsCheckBox
    }
}

pub struct WindowsFactoryDyn;
impl GuiFactoryDynamic for WindowsFactoryDyn {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }

    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(WindowsCheckBox)
    }
}

pub struct WindowsButton;
impl Button for WindowsButton{
    fn press(&self) {
        println!("Windows按钮触发!");
    }
}

pub struct WindowsCheckBox;
impl CheckBox for WindowsCheckBox {
    fn switch(&self) {
        println!("Windows切换按钮触发");
    }
}