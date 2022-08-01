use hex;
use std::str;

fn process_stream(mut encoded_str: &str, code: u64){
    let str_start_time : &str = &encoded_str[..16];
    let start_time : u64 = u64::from_str_radix(str_start_time, 16).unwrap();
    println!("start time is = {}", start_time);   
    encoded_str = &encoded_str[16..encoded_str.len()];

    let str_end_time : &str = &encoded_str[..16];
    let end_time : u64 = u64::from_str_radix(str_end_time, 16).unwrap();
    println!("end time is = {}", end_time);   
    encoded_str = &encoded_str[16..encoded_str.len()];

    let str_amount : &str = &encoded_str[..16];
    let amount : u64 = u64::from_str_radix(str_amount, 16).unwrap();
    println!("amount is = {}", amount); 
    encoded_str = &encoded_str[16..encoded_str.len()];
    
    let str_chain_id : &str = &encoded_str[..4];
    encoded_str = &encoded_str[4..encoded_str.len()];
    let str_sender : &str = &encoded_str[..40];
    let str_receiver : &str = &encoded_str[40..encoded_str.len()];
    let header : &str = "0x";
    let sender = format!("{}{}", header, str_sender);
    let receiver = format!("{}{}", header, str_receiver);
    let from_chain_id : u64 = u64::from_str_radix(str_chain_id, 16).unwrap();

    println!("from chain_id is = {}", from_chain_id);
    println!("sender is = {}", sender);
    println!("receiver is = {}", receiver);

    if code == 1 {
        println!("Sol Stream");
    }
    if code == 2 {
        println!("Token Stream");
    }      
}

fn process_withdraw_stream(mut encoded_str: &str, code: u64){
    let str_amount : &str = &encoded_str[..16];
    let amount : u64 = u64::from_str_radix(str_amount, 16).unwrap();
    println!("amount is = {}", amount); 

    encoded_str = &encoded_str[16..encoded_str.len()];
    let str_chain_id : &str = &encoded_str[..4];
    let str_withdrawer : &str = &encoded_str[4..encoded_str.len()];
    let header : &str = "0x";
    let withdrawer = format!("{}{}", header, str_withdrawer);
    let from_chain_id : u64 = u64::from_str_radix(str_chain_id, 16).unwrap();

    println!("from chain_id is = {}", from_chain_id);
    println!("withdrawer is = {}", withdrawer);

    if code == 3 {
        println!("Sol Withdraw Stream");
    }
    if code == 4 {
        println!("Token Withdraw Stream");
    }
}

fn process_deposit(mut encoded_str: &str, code: u64){
    let str_amount : &str = &encoded_str[..16];
    let amount : u64 = u64::from_str_radix(str_amount, 16).unwrap();
    println!("amount is = {}", amount); 

    encoded_str = &encoded_str[16..encoded_str.len()];
    let str_chain_id : &str = &encoded_str[..4];
    let str_depositer : &str = &encoded_str[4..encoded_str.len()];
    let header : &str = "0x";
    let depositer = format!("{}{}", header, str_depositer);
    let from_chain_id : u64 = u64::from_str_radix(str_chain_id, 16).unwrap();

    println!("from chain_id is = {}", from_chain_id);
    println!("depositer is = {}", depositer);

    if code == 5 {
        println!("Sol Deposit");
    }
    if code == 6 {
        println!("Token Deposit");
    }
}

fn process_fund(mut encoded_str: &str, code: u64){
    let str_end_time : &str = &encoded_str[..16];
    let end_time : u64 = u64::from_str_radix(str_end_time, 16).unwrap();
    println!("end time is = {}", end_time);   
    encoded_str = &encoded_str[16..encoded_str.len()];

    let str_amount : &str = &encoded_str[..16];
    let amount : u64 = u64::from_str_radix(str_amount, 16).unwrap();
    println!("amount is = {}", amount); 
    encoded_str = &encoded_str[16..encoded_str.len()];
   
    let str_chain_id : &str = &encoded_str[..4];
    let str_sender = &encoded_str[4..encoded_str.len()];
    let header : &str = "0x";
    let sender = format!("{}{}", header, str_sender);
    let from_chain_id : u64 = u64::from_str_radix(str_chain_id, 16).unwrap();

    println!("from chain_id is = {}", from_chain_id);
    println!("sender is = {}", sender);

    if code == 7 {
        println!("Sol Fund");
    }
    if code == 8 {
        println!("Token Fund");
    }
}

fn process_withdraw(mut encoded_str: &str, code: u64){
    let str_amount : &str = &encoded_str[..16];
    let amount : u64 = u64::from_str_radix(str_amount, 16).unwrap();
    println!("amount is = {}", amount); 

    encoded_str = &encoded_str[16..encoded_str.len()];
    let str_chain_id : &str = &encoded_str[..4];
    let str_withdrawer : &str = &encoded_str[4..encoded_str.len()];
    let header : &str = "0x";
    let withdrawer = format!("{}{}", header, str_withdrawer);
    let from_chain_id : u64 = u64::from_str_radix(str_chain_id, 16).unwrap();

    println!("from chain_id is = {}", from_chain_id);
    println!("withdrawer is = {}", withdrawer); 

    if code == 9 {
        println!("Sol Withdraw");
    }
    if code == 10 {
        println!("Token Withdraw");
    }
}

fn process_swap(mut encoded_str: &str, code: u64){
    let str_amount : &str = &encoded_str[..16];
    let amount : u64 = u64::from_str_radix(str_amount, 16).unwrap();
    println!("amount is = {}", amount); 

    encoded_str = &encoded_str[16..encoded_str.len()];
    let str_chain_id : &str = &encoded_str[..4];
    let str_sender : &str = &encoded_str[4..encoded_str.len()];
    let header : &str = "0x";
    let sender = format!("{}{}", header, str_sender);
    let from_chain_id : u64 = u64::from_str_radix(str_chain_id, 16).unwrap();

    println!("from chain_id is = {}", from_chain_id);
    println!("sender is = {}", sender);

    if code == 11 {
        println!("Sol Swap");
    }
    if code == 12 {
        println!("Token Swap");
    }
}

fn main() {

    // TESTING STRING
    let encoded_string : String = hex::encode("helloworld");
    // let encoded_string = "0x68656c6c6f776f726c64";
    // will panic for 0x need to remove them before decoding
    println!("{:?}", encoded_string);
    let decoded_bytes : Vec<u8> = hex::decode(encoded_string).unwrap();
    println!("{:?}", decoded_bytes);
    let original_string : &str = str::from_utf8(&decoded_bytes).unwrap();
    println!("{:?}", original_string);
    //println!("{:?}", "helloworld".to_owned().into_bytes());

    // TESTING PAYLOAD
    let process_stream_payload : &[u8; 136] = b"0x0100000000000000010000000000000002000000000000000300045b38da6a701c568545dcfcb03fcb875f56beddc4ab8483f64d9c6d1ecf9b849ae677dd3315835cb2";
    let process_withdraw_streamm : &[u8; 64] = b"0x0300000000000004d200015b38da6a701c568545dcfcb03fcb875f56beddc4";
    let process_deposit_payload : &[u8; 64] = b"0x0500000000000004d200015b38da6a701c568545dcfcb03fcb875f56beddc4";
    let process_fund_payload : &[u8; 80] = b"0x070000000000087a2300000000000004d200015b38da6a701c568545dcfcb03fcb875f56beddc4";
    let process_withdraw_payload : &[u8; 64] = b"0x0900000000000004d200015b38da6a701c568545dcfcb03fcb875f56beddc4";
    let process_swap_payload: &[u8; 64] = b"0x0c00000000000004d200015b38da6a701c568545dcfcb03fcb875f56beddc4";
    
    let payload = process_stream_payload;
    let temp : &str = str::from_utf8(payload).unwrap();
    
    let mut encoded_str : &str = &temp[2..temp.len()];
    //println!("{} = ", encoded_str);
    
    let str_code : &str = &encoded_str[..2];
    let code : u64 = u64::from_str_radix(str_code, 16).unwrap();
    println!("code is = {}", code);
    encoded_str = &encoded_str[2..encoded_str.len()];
           
    // Switch based on code
    match code {
        1|2 =>
            //process (sol or token) stream
            process_stream(encoded_str, code),
        3|4 =>
            //process (sol or token) withdraw stream
            process_withdraw_stream(encoded_str, code),
        5|6 => 
            //deposit (sol or token) sol
            process_deposit(encoded_str, code),            
        7|8 => 
            //fund (sol or token) sol
            process_fund(encoded_str, code),
        9|10 => 
            //withdraw (sol or token) sol
            process_withdraw(encoded_str, code),
        11|12 => 
            //swap (sol or token)
            process_swap(encoded_str, code),
        _ => 
            //throw error
            println!("error"),
    } 
}
