// Importing the external crate (library) into your environment
extern crate flate2;

// flate2 is an external rust library that provides support for compression and decompression of files 

// use keyword is used to import modules from a crate (library)
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;


// the main function
fn main(){
    println!("Start");
    if args().len() != 3{
        println!("args length -> {}" , args().len());
        println!("Usage: 'Source' 'target'" );
        return ;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    println!("Input file has been obtained");

    let output = File::create(args().nth(2).unwrap()).unwrap();


    let mut encoder = GzEncoder::new(output , Compression::default());
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();

    println!("File creation was successful");

    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len:{:?}" , output.metadata().unwrap().len());
    println!("elapsed: {:?}" , start.elapsed());
}