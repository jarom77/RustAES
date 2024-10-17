mod sbox;
mod arr_functions;

const WORDS_IN_KEY: u16 = 4;
pub const BITS_IN_WORD: u16 = 32;

pub fn round(in_data: u128, roundkey: u128, i: u8, nr: u8, verbose: bool) -> u128 {
    // convert input to 4x4 byte array
    let mut state: [[u8; 4]; 4] = [[0u8; 4]; 4];
    to_array(&mut state, in_data);
    
    // run sbox
    sbox::forward(&mut state);
    if verbose {
        println!("round[{:>2}].s_box     {:0>32x}", i, from_array(&state));
    }

    // shift rows
    arr_functions::shift_rows(&mut state);
    if verbose {
        println!("round[{:>2}].s_row     {:0>32x}", i, from_array(&state));
    }
    
    // mix column
    if i != nr {
        arr_functions::mix_columns(&mut state);
        if verbose {
            println!("round[{:>2}].m_col     {:0>32x}", i, from_array(&state));
        }
    }

    // convert back to 128-byte state and xor with key
    roundkey ^ from_array(&state)
}

pub fn inv_round(in_data: u128, roundkey: u128, i: u8, nr: u8, verbose: bool) -> u128 {
    let mut arr_state: [[u8; 4]; 4] = [[0u8; 4]; 4];
    to_array(&mut arr_state, in_data);
    arr_functions::inv_shift_rows(&mut arr_state);
    let shifted = from_array(&arr_state);
    let sub_state: u128 = sbox::inverse(shifted);
    let mut state: u128 = sub_state ^ roundkey;

    if verbose {
        println!("round[{:>2}].is_row    {:0>32x}", i, shifted);
        println!("round[{:>2}].is_box    {:0>32x}", i, sub_state);
        println!("round[{:>2}].ik_sch    {:0>32x}", i, roundkey);
    }

    if i != nr {
        if verbose {
            println!("round[{:>2}].ik_add    {:0>32x}", i, state);
        }
        // to_array(&mut arr_state, state);
        // arr_functions::old_inv_mix_columns(&mut arr_state);
        // state = from_array(&arr_state);

        state = arr_functions::inv_mix_columns(state);
    }

    state
}

pub fn round_key(key: &mut [u128; 2], round: u8, keylen: u16, rcon: &mut u8, verbose: bool) -> u128 {
    let mut new_key: u128;
    if round == 0 {
        new_key = key[0];
        *rcon = 1;
    } else if round == 1 && keylen == 256 {
        new_key = key[1];
    } else {
        let nk: u16 = keylen/BITS_IN_WORD;
        let mut word: u16 = (round as u16)*WORDS_IN_KEY;
        let mut i: u16 = 0;
        if round == 1 {
            if keylen == 192 {
                word = keylen/BITS_IN_WORD;
                i = word - WORDS_IN_KEY;
                new_key = key[1];
            } else { new_key = key[0] };
            key[1] = key[0];
        } else { new_key = key[1] };

        let mut temp: u32 = new_key as u32;
        while i < WORDS_IN_KEY {
            if verbose {
                print!("{:0>2} | {:0>8x} | ", word, temp);
            }
            // create next word, save in new_key
            if word % nk == 0 {
                let rot_word: u32 = temp << 8 | temp >> 24;
                let sub_word: u32 = sbox::sub_word(rot_word);
                let rconword: u32 = (*rcon as u32) << 24;
                temp = sub_word ^ rconword;
                if verbose {
                    print!("{:0>8x} | {:0>8x} | {:0>8x} | {:0>8x} | ", rot_word, sub_word, rconword, temp);
                }
                *rcon = arr_functions::xtime(*rcon);
            } else if nk > 6 && word % nk == 4 {
                temp = sbox::sub_word(temp);
                if verbose {
                    print!("         | {:0>8x} |          |          | ", temp);
                }
            } else if verbose {
                print!("         |          |          |          | ");
            }

            // put in new word
            let prev_word: u32 = get_word(i as i16 - nk as i16 + 1, &key);
            temp ^= prev_word;
            if verbose {
                println!("{:0>8x} | {:0>8x}", prev_word, temp);
            }
            new_key <<= BITS_IN_WORD;
            new_key |= temp as u128;

            // increment to next word, reassign temp
            word += 1;
            i += 1;
        }

        key[0] = key[1];
        key[1] = new_key;
    }
    new_key
}

fn get_word(i: i16, key: &[u128; 2]) -> u32 {
    // println!("{:x} >> {}", key[1], ((BITS_IN_WORD as i16) * -1*i));
    // println!("secondary: {}", -(i + WORDS_IN_KEY as i16));
    // println!("i={}", i);
    if i > -(WORDS_IN_KEY as i16) {
        let offset: usize = ((BITS_IN_WORD as i16) * -1*i) as usize;
        (key[1] >> offset) as u32
    } else {
        (key[0] >> (BITS_IN_WORD * -(i + WORDS_IN_KEY as i16) as u16)) as u32
    }
}

fn to_array(state: &mut [[u8; 4]; 4], data: u128) {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = (data >> (15 - j * 4 - i) * 8) as u8;
        }
    }
}

fn from_array(arr: &[[u8; 4]; 4]) -> u128 {
    let mut state: u128 = 0;
    for i in 0..4 {
        for j in 0..4 {
            state = state << 8 | arr[j][i] as u128;
        }
    }
    state
}
