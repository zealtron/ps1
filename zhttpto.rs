//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};
use std::io::buffered::BufferedReader;
use std::io::File;

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut visitor_count: int = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {
				    println(format!("Received connection from: [{:s}]", pn.to_str()));
				    unsafe {visitor_count += 1;}
				},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
	    unsafe{println(format!("{:d} visitors", visitor_count));}
            println(format!("Received request :\n{:s}", request_str));
	    let r: ~[&str] = request_str.split(' ').collect();
	    if (r[0].contains("GET") && r[2].contains("HTTP/1.1")) {
		let response: ~str = request(r[1]);
		stream.write(response.as_bytes());

	    }
	    else {
		//print("TEST");
 		let response: ~str = 
                    ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                     <doctype !html><html><head><title>Hello, Rust!</title>
                     <style>body { background-color: #111; color: #FFEEAA }
                            h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                            h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
               	     </style></head>
                     <body>
                     <h1>Greetings, Krusty!</h1>
                     </body></html>\r\n";
		stream.write(response.as_bytes());
	    }
		    
	    
	    
            
            //let response: ~str = 
              //  ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                // <doctype !html><html><head><title>Hello, Rust!</title>
                 //<style>body { background-color: #111; color: #FFEEAA }
                  //      h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                    //    h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 //</style></head>
                 //<body>
                 //<h1>Greetings, Krusty!</h1>
                 //</body></html>\r\n";
            //stream.write(response.as_bytes());
            println!("Connection terminates.");
        }
    }
}

fn request(a: &str) -> ~str{
    //print!("{:s}", a);
    let path = Path::new("/home/student/ps1" + a);
    if (path.is_file() && a.contains(".html")){
	let file = File::open(&path);
	let mut reader = BufferedReader::new(file);
	let contents = reader.read_to_str();
	let response: ~str = 
                    ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                     <doctype !html><html><head><title>Hello, Rust!</title>
                     <style><!--
				body {font-family: arial,sans-serif; text-align: center}
				img { border:none; }
				//-->
               	     </style></head>
                     <body>
                     <h2>" + contents + "</h2>
                     </body></html>\r\n";
	return response;
    } 
    else if path.is_file() {
	let response: ~str = 
                    ~"HTTP/1.1 403 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                     <doctype !html><html><head><title>Error 403 Forbidden</title>
                     <style><!--
				body {font-family: arial,sans-serif}
				img { border:none; }
				//-->
               	     </style></head>
                     <body>
		     <blockquote>
		     <h2>Error 403 Forbidden</h2>
		     <p>You don't have permission to acces this on this server.
		     </blockquote>
     		     </body></html>\r\n";
	return response;
	
    }
    else {
        //println("NO FILE EXISTS");
	let response: ~str = 
                    ~"HTTP/1.1 404 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                     <doctype !html><html><head><title>Error 404 Not Found</title>
                     <style><!--
				body {font-family: arial,sans-serif}
				img { border:none; }
				//-->
               	     </style></head>
                     <body>
		     <blockquote>
		     <h2>Error 404 NOT FOUND</h2>
		     <p>Our apologies for the temporary inconvenience. The requested URL was not found on this server.
		     </blockquote>
     		     </body></html>\r\n";
	return response;
    }

}
