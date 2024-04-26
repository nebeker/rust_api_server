use std::{io::Read, io::Write, net::TcpStream, thread};

enum ServerError {
    NotFound,
}

fn get_response(request: &str) -> String {
    prepare_response(route(request))
}

fn route(request: &str) -> Result<String, ServerError> {
    if request.contains("GET /api") {
        Ok(get_data())
    } else {
        Err(ServerError::NotFound)
    }
}

fn prepare_response(result: Result<String, ServerError>) -> String {
    match result {
        Ok(text) => format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
            text
        ),
        Err(error) => match error {
            ServerError::NotFound => String::from("HTTP/1.1 404 Not Found\r\nNot Found"),
            _ => String::from("HTTP/1.1 500 Internal Server Error\r\nInternal Server Error"),
        },
    }
}

fn get_data() -> String {
    let (key, value) = ("working", true);
    format!("{{\r\n\"{}\":\"{}\"\r\n}}", key, value)
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connected to client: {stream:?}");
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            print!("request: {}", request);
            let response = get_response(&request);
            println!("response: {response}");
            match stream.write_all(response.as_bytes()) {
                Ok(_) => match stream.flush() {
                    Ok(_) => println!("Sent response to client"),
                    Err(e) => eprintln!("Failed to send response to client: {e:?}"),
                },
                Err(e) => eprintln!("Failed to send response to client: {e:?}"),
            }
        }
        Err(e) => {
            eprintln!("Error: {e:?}")
        }
    }
}
fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Listening on http://127.0.0.1:8080 - Ctrl+C to stop");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => println!("Failed to connect to client: {e:?}"),
        }
    }
}
