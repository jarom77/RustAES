mod aes;

/* AES single-block cipher
args:
    -e/d
    data
    key
    type
    v_lvl
*/

const SYNTAX_HINT: &str = "args: -[e|d][v] data key type";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    assert!(args[1][0..1] == *"-", "{}", SYNTAX_HINT);
    let mut verbose: bool = false;
    if args[1].find('v') != None { verbose = true };
    let data: u128 = u128::from_str_radix(&args[2], 16).unwrap();
    let mykey: [u128; 2] = [u128::from_str_radix(&args[3][..32], 16).unwrap(), match u128::from_str_radix(&args[3][32..], 16){ Ok(a) => a, Err(_) => 0}];
    let keylen = args[4].parse().unwrap();

    if args[1].find('e') != None {
        println!("{:0>32x}", aes::encrypt(data, &mykey, keylen, verbose));
    } else if args[1].find('d') != None {
        println!("{:0>32x}", aes::decrypt(data, &mykey, keylen, verbose));
    } else {
        println!("Option not recognized.\n{}", SYNTAX_HINT);
    }
}
