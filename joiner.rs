use std::rand::random;
use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile1> <inputfile2>", args[0]); 
    } else {
        let fname1 = args[1];
	let args2: ~[~str] = os::args();
	let fname2 = args2[1];
        let path1 = Path::new(fname1.clone());
        let path2 = Path::new(fname2.clone());
        let msg_file1 = File::open(&path1);
        let msg_file2 = File::open(&path2);

        match (msg_file1) {
            Some(mut msg1) => {
		match (msg_file2) {
		    Some(mut msg2) => {
                        let msg_bytes1: ~[u8] = msg1.read_to_end();
		        let msg_bytes2: ~[u8] = msg2.read_to_end();
                        let share_file = File::create(&Path::new("msg.share"));
                
                        match (share_file) {
                            Some(share) => { 
                                join(msg_bytes1, msg_bytes2, share); 
                                } ,
                            None => fail!("Error opening output files!"),
                        }
		    } ,
		    None => fail!("Error opening message file: {:s}", fname2)
		}
            } ,
            None => fail!("Error opening message file: {:s}", fname1)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}

fn join(msg_bytes1: &[u8], msg_bytes2: &[u8], mut share: File) {
    //let mut random_bytes: ~[u8] = ~[];
    // This is not cryptographically strong randomness! 
    // (For entertainment purposes only.)
    //for _ in range(0, msg_bytes.len()) {
    //	let random_byte = random();
    //	random_bytes.push(random_byte);
    //}
    
    let msg = xor(msg_bytes1, msg_bytes2);
    share.write(msg);
}
