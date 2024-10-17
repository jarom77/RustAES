mod aes_round;

const VERB_KEY: bool = false;

pub fn encrypt(plaintext: u128, user_key: &[u128; 2], keylen: u16, verbose: bool) -> u128 {
	if verbose {
        println!("round[{:>2}].input     {:0>32x}", 0, plaintext);
    }
    let mut key: [u128; 2] = *user_key;
    let mut round_text: u128 = plaintext;
    let nr: u8 = (keylen/aes_round::BITS_IN_WORD + 6) as u8;
	let mut rcon: u8 = 0;

    let mut round_key = aes_round::round_key(&mut key, 0, keylen, &mut rcon, VERB_KEY);
    if verbose {
        println!("round[{:>2}].k_sch     {:0>32x}", 0, round_key);
    }
    round_text ^= round_key;

    // begin round
    for i in 1..(nr+1) {
        if verbose {
            println!("round[{:>2}].start     {:0>32x}", i, round_text);
        }
        round_key = aes_round::round_key(&mut key, i, keylen, &mut rcon, VERB_KEY);
        round_text = aes_round::round(round_text, round_key, i, nr, verbose);
        if verbose {
            println!("round[{:>2}].k_sch     {:0>32x}", i, round_key);
        }
    }
    if verbose {
        println!("round[{:>2}].output    {:0>32x}", nr, round_text);
    }

    round_text
}

pub fn decrypt(ciphertext: u128, user_key: &[u128; 2], keylen: u16, verbose: bool) -> u128 {
	let mut key: [u128; 2] = *user_key;
    let nr: usize = (keylen/aes_round::BITS_IN_WORD + 6) as usize;
	let mut rcon: u8 = 0;

	// compute all keys
	let mut round_keys: [u128; 15] = [0; 15];
	for i in 0..(nr+1) {
    	round_keys[nr-i] = aes_round::round_key(&mut key, i as u8, keylen, &mut rcon, VERB_KEY);
	}

	// create mutable copy of ciphertext
	let mut round_text: u128 = ciphertext;
	round_text ^= round_keys[0];

	if verbose {
        println!("round[{:>2}].iinput    {:0>32x}", 0, ciphertext);
		println!("round[{:>2}].ik_sch    {:0>32x}", 0, round_keys[0]);
    }

	for i in 1..(nr+1) {
		if verbose {
			println!("round[{:>2}].istart    {:0>32x}", i, round_text);
		}

		round_text = aes_round::inv_round(round_text, round_keys[i], i as u8, nr as u8, verbose);
	}

    if verbose {
        println!("round[{:>2}].ioutput   {:0>32x}", nr, round_text);
    }
	round_text
}
