use async_std::task;
use async_std::task::JoinHandle;
use std::thread;
use std::env;
use std::ffi::OsStr;

static ASYNC_STD_THREAD_COUNT: u32 = 4;

use crate::fastq::index_fastq;

async fn index_fastq_threaded(url: &str,  dir: &str) -> Result<String, String> {
    println!("Indexing file on thread {:?}", thread::current().id());
    return index_fastq(url, dir);
}



pub fn index_fastq_list(fastq_files: Vec<&str>, dir: &str) -> Vec<String> {

    env::set_var("ASYNC_STD_THREAD_COUNT", OsStr::new(ASYNC_STD_THREAD_COUNT.to_string().as_str()));


    let mut tasks = Vec::with_capacity(fastq_files.len());

    for fastq_file_path in fastq_files.iter() {
        let url = fastq_file_path.to_string();

        let x = String::from(dir);

        let handle: JoinHandle<_> = task::spawn(async move {
                
            let fq = index_fastq_threaded(&url.as_str(), x.as_str()).await;
            return fq;
        });

        tasks.push(handle);
    }

    let mut v: Vec<String> = Vec::new();


    task::block_on(async {
        for t in tasks {
            let r: Result<String, String> = t.await;

            match r {
                Ok(_body) => { 
                    println!("result == {:?}", _body);
                    v.push(_body);
                },
                Err(e) => println!("error {:?}", e),
            }

            
        }
    });

    return v;


}