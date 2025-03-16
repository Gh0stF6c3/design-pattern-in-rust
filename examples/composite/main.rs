
// 模式类型: 结构性
// 设计模式: 组合
// 适用场景: 用于将对象组合成树形结构，以表示“部分-整体”的层次结构。该模式让客户端可以
//          一致地对待单个对象和对象组合，而无需关心它们是单个对象还是多个对象的组合

mod fs;

use fs::{Component,File,Folder};

fn main() {
    let file1 = File::new("File-1");
    let file2 = File::new("File-2");
    let file3 = File::new("File-3");

    let mut folder1 = Folder::new("Folder-1");
    folder1.add(file1);

    let mut folder2 = Folder::new("Folder-2");
    folder2.add(file2);
    folder2.add(file3);

    folder1.search("Hello");
    folder2.search("World");
    
}