use async_std::task;
use async_std::task::JoinHandle;
use std::env;
use std::ffi::OsStr;
use crate::fastq::index_fastq;

// static ASYNC_STD_THREAD_COUNT: u32 = 4;

async fn index_fastq_threaded(fastq_file_path: &str,  dir: &str) -> Result<String, String> {
    //rprintln!("Indexing fastq on thread {:?}", thread::current().id());
    return index_fastq(fastq_file_path, dir);
}



pub fn index_fastq_list(fastq_files: Vec<&str>, dir: &str, threads: u8) -> Vec<String> {

    //env::set_var("ASYNC_STD_THREAD_COUNT", OsStr::new(ASYNC_STD_THREAD_COUNT.to_string().as_str()));
    env::set_var("ASYNC_STD_THREAD_COUNT", OsStr::new(threads.to_string().as_str()));

    let mut tasks = Vec::with_capacity(fastq_files.len());

    for fastq_file_path in fastq_files.iter() {
        let my_fastq = fastq_file_path.to_string();
        let my_dir = String::from(dir);

        let handle: JoinHandle<_> = task::spawn(async move {
            let fq = index_fastq_threaded(&my_fastq.as_str(), my_dir.as_str()).await;
            return fq;
        });

        tasks.push(handle);
    }

    let mut indexing_results: Vec<String> = Vec::new();

    task::block_on(async {
        for t in tasks {
            let r: Result<String, String> = t.await;
            match r {
                Ok(body) => { 
                    indexing_results.push(body);
                },
                Err(e) => println!("error {:?}", e),
            }
        }
    });
    return indexing_results;
}