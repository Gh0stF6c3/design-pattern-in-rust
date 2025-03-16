
// 模式类型: 行为模式
// 设计模式: 命令
// 适用场景: 将请求转化为一个包含与请求相关的所有信息的独立对象，这种转换能够让开发者实现可撤销操作。
//          这个示例模拟了一个文本编辑器，使用命令行模式对逻辑代码进行优化和撤销。

// 关键点     1. 示例中每个按钮包含了不同的执行逻辑
//           2. 命令模式中命令操作被封装为了一个对象，所以可以像操作数组一样对目标进行移除

use cmd::{copy::CopyCommand, cut::CutCommand, paste::PasteCommand, Command};
use cursive::{view::Nameable, views::{Dialog, EditView}, Cursive};

mod cmd;


// 应用上下文
#[derive(Default)]
struct AppContext {
    clipboard: String,
    history: Vec<Box<dyn Command>>
}

fn main() {
    let mut app = cursive::default();

    app.set_user_data(AppContext::default());
    app.add_layer(
        Dialog::around(EditView::default().with_name("Editor"))
        .title("命令模式演示")
        .button("复制", |s| execute(s, CopyCommand))
        .button("剪切", |s| execute(s, CutCommand::default()))
        .button("粘贴", |s| execute(s, PasteCommand::default()))
        .button("撤销操作", undo)
        .button("退出", |s| s.quit())   
    );

    app.run();
}       

fn execute(app: &mut Cursive,mut command:impl Command + 'static) {
    if command.execute(app) {
        app.with_user_data(|context: &mut AppContext| {
            context.history.push(Box::new(command));
        }); 
    }
}

fn undo(app: &mut Cursive) {
    let mut context = app.take_user_data::<AppContext>().unwrap();
    if let Some(mut command) = context.history.pop() {
        command.undo(app);
    }

    app.set_user_data(context);
}