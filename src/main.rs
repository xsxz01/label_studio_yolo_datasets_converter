use clap::{Arg, Command};
use label_studio_yolo_datasets_converter::converter::handler::{deal_path, deal_zip};

fn main() {
    // 获取参数
    let args = cli().get_matches();
    // 获取参数值
    let type_ = args.get_one::<String>("type").unwrap();
    let path = args.get_one::<String>("path").unwrap();
    let output = args.get_one::<String>("output").unwrap();
    match type_.as_str() {
        "zip" => {
            // 处理文件
            println!("{}", path);
            // 处理zip文件
            deal_zip(path, output);
        }
        "path" => {
            // 处理文件夹
            println!("{}", path);
            deal_path(path, output);
        }
        _ => {
            println!("type must be zip or path");
        }
    }
}

/**
* 使用clap接受参数
*  type    文件类型    文件或文件夹,位置参数
*  path    文件路径    文件夹路径或者，位置参数
*  output  输出路径    文件夹路径，必填参数
*  调用如下
*  label_studio_yolo_datasets_converter.exe type path
*/
fn cli() -> Command {
    let command = Command::new("label_studio_yolo_datasets_converter");
    command
        .about("label_studio_yolo_datasets_converter")
        .version("0.0.1")
        .author("Pang")
        .arg(Arg::new("type").help("文件类型, 有效值zip, path").required(true).index(1))
        .arg(Arg::new("path").help("文件路径").required(true).index(2))
        .arg(
            Arg::new("output")
                .help("输出路径")
                .required(true)
                .short('o')
                .long("output"),
        )
}
