use std::time;
use std::time::Duration;
use std::time::SystemTime;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
fn main() {
    let mut word  = String::new();
    let result = translate(word);
    print!("{}",result);
    // 创建剪贴板上下文
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let ms_300 = time::Duration::from_millis(300);
    let mut last_word = "".to_string();
    loop {
        std::thread::sleep(ms_300);
        // 读取剪贴板的文本内容
        if let Ok(contents) = ctx.get_contents() {
            if contents == last_word{
                // println!("your paste is not fresh");
            }else{
                last_word = contents.clone();
                println!("剪贴板内容: {}", contents.clone());
            }
        } else {
            println!("无法读取剪贴板");
        }
    }


}
fn translate(word:String) -> String {
    let result: String = word + "111";
    result
}
