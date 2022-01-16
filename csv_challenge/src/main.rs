use std::path::PathBuf;
use std::process;
use structopt::StructOpt;
use crate::core::read::{load_csv, write_csv};
use crate::core::write::replace_column;
use crate::err::Error;
use crate::opt::Opt;

mod opt;
mod err;
mod core;

fn main() {
    let opt: Opt = Opt::from_args();
    let filename = PathBuf::from(opt.input);

    // 读取内容
    let csv_data = match load_csv(filename) {
        Ok(file_context) => {
            println!("原csv文件内容为：{:?}",file_context);
            file_context
        }
        Err(e) => {
            println!("main error: {:?}",e);
            process::exit(1);
        }
    };

    // 替换内容
    let modified_data = match replace_column(csv_data,&opt.column_name,&opt.replacement) {
        Ok(data) => {data}
        Err(e) => {
            println!("main error: {:?}",e);
            process::exit(1);
        }
    };

    // 写入内容
    let output_file = &opt.output.unwrap_or("output/output.csv".to_string());
    match write_csv(&modified_data,&output_file) {
        Ok(_) => {
            println!("写入文件成功！");
        }
        Err(e) => {
            println!("main error: {:?}",e);
            process::exit(1);
        }
    }

    //println!("{:#?}", opt);
}
