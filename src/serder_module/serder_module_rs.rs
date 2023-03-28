use std::net::{TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::{str};

use pyo3::prelude::pyfunction;

#[derive(Serialize, Deserialize, Debug)]
struct MessageSerialized {
    value: f64,
    best_vector: Vec<f64>
}

#[pyfunction]
pub fn send_request_py(id_addr: &str, value: f64, best_vector: Vec<f64>) -> Vec<f64>{
    let mut stream = TcpStream::connect(id_addr).expect("Could not connect to server");

    let mut buffer: Vec<u8> = Vec::new();
    let point = MessageSerialized {
        value: value,
        best_vector: best_vector,
        
    };
    stream
        .write_all(serde_json::to_string(&point).unwrap().as_bytes())
        .expect("Failed to write to server");
    stream.write_all(b"\n").expect("Failed to write to server");

    let mut reader = BufReader::new(&stream);
    reader
        .read_until(b'\n', &mut buffer)
        .expect("Could not read into buffer");
    let input = str::from_utf8(&buffer).expect("Could not write buffer as string");
    if input == "" {
        eprintln!("Empty response from server");
    }

    let mi_input = input.replace('\n', "").split(",").filter_map(|s| s.parse::<f64>().ok()).collect::<Vec<_>>();

    return mi_input;

}

pub fn manda_mensage(){
    let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to server");

    println!("Please provide a 3D point as three comma separated integers");
    
    let mut buffer: Vec<u8> = Vec::new();
    let point = MessageSerialized {
        value: 3452.4545,
        best_vector: vec![2.9, 3.8, 4.7],
        
    };
    stream
        .write_all(serde_json::to_string(&point).unwrap().as_bytes())
        .expect("Failed to write to server");
    stream.write_all(b"\n").expect("Failed to write to server");

    let mut reader = BufReader::new(&stream);
    reader
        .read_until(b'\n', &mut buffer)
        .expect("Could not read into buffer");
    let input = str::from_utf8(&buffer).expect("Could not write buffer as string");
    if input == "" {
        eprintln!("Empty response from server");
    }
    print!("Response from server {}", input);

}