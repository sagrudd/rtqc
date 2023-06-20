use async_std::task;
use std::thread;
use std::env;
use std::ffi::OsStr;

static ASYNC_STD_THREAD_COUNT: u32 = 4;

use crate::fastq::index_fastq;

async fn index_fastq_threaded(url: &str,  dir: &str) -> Result<String, String> {
    println!("Indexing file on thread {:?}", thread::current().id());
    return index_fastq(url, dir);
}



pub fn index_fastq_list(fastq_files: Vec<&str>, dir: &str) {

    env::set_var("ASYNC_STD_THREAD_COUNT", OsStr::new(ASYNC_STD_THREAD_COUNT.to_string().as_str()));

    let mut tasks = Vec::with_capacity(fastq_files.len());


    for fastq_file_path in fastq_files.iter() {
        let url = fastq_file_path.to_string();

        let x = String::from(dir);
        
        tasks.push(
            task::spawn(async move {
                
                match index_fastq_threaded(&url.as_str(), x.as_str()).await {
                    Ok(_body) => { 
                        // Do something useful 

                    },
                    Err(e) => println!("    Got error {:?}", e),
                }
            })
        )
    }

    task::block_on(async {
        for t in tasks {
            t.await;
        }
    });
}