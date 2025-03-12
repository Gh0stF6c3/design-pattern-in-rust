use crate::gui::{Button, CheckBox, GuiFactory, GuiFactoryDynamic};

pub struct MacOSFactory;
impl GuiFactory for MacOSFactory {
    type B = MacOSButton;
    type C = MacOSCheckBox;

    fn create_button(&self) -> Self::B {
        MacOSButton
    }

    fn create_checkbox(&self) -> Self::C {
        MacOSCheckBox
    }
}

pub struct MacOSFactoryDyn;
impl GuiFactoryDynamic for MacOSFactoryDyn {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacOSButton)
    }

    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(MacOSCheckBox)
    }
}

pub struct MacOSButton;
impl Button for MacOSButton {
    fn press(&self) {
        println!("MacOS按钮触发!")
    }
}

pub struct MacOSCheckBox;
impl CheckBox for MacOSCheckBox {
    fn switch(&self) {
        println!("MacOS切换按钮触发!")
    }
}