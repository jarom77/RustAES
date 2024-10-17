pub fn shift_rows(state: &mut [[u8; 4]; 4]) {
	let mut row = [0u8; 4];
	for i in 1..4 {
		for j in 0..4 {
			row[j] = state[i][(j + i) % 4];
		}
		for j in 0..4 {
			state[i][j] = row[j];
		}
	}
}

pub fn inv_shift_rows(state: &mut [[u8; 4]; 4]) {
	let mut row = [0u8; 4];
	for i in 1..4 {
		for j in 0..4 {
			row[j] = state[i][(j + 4 - i) % 4];
		}
		for j in 0..4 {
			state[i][j] = row[j];
		}
	}
}

pub fn mix_columns(state: &mut [[u8; 4]; 4]) {
	let mut column = [0u8; 4];
	for c in 0..4 {
		for r in 0..4 {
			column[r] = xtime(state[r][c]) 
			^ state[(r + 1) % 4][c] ^ xtime(state[(r + 1) % 4][c]) 
			^ state[(r + 2) % 4][c] 
			^ state[(r + 3) % 4][c];
		}
		for r in 0..4 {
			state[r][c] = column[r]
		}
	}
}

pub fn inv_mix_columns(state: u128) -> u128 {
	let mut mixed: u128 = 0;
	for i in [3, 2, 1, 0] {
		let col: u32 = (state >> i * 32) as u32;
		let orig: [u8; 4] = [(col >> 24) as u8, (col >> 16) as u8, (col >> 8) as u8, col as u8];
		// mixed <<= 8; mixed |= invmix_matmul(orig[0], orig[1], orig[2], orig[3]) as u128;
		// mixed <<= 8; mixed |= invmix_matmul(orig[1], orig[2], orig[3], orig[0]) as u128;
		// mixed <<= 8; mixed |= invmix_matmul(orig[2], orig[3], orig[0], orig[1]) as u128;
		// mixed <<= 8; mixed |= invmix_matmul(orig[3], orig[0], orig[1], orig[2]) as u128;

		mixed <<= 8; mixed |= (ff_mult(0xe, orig[0]) ^ ff_mult(0xb, orig[1]) ^ ff_mult(0xd, orig[2]) ^ ff_mult(0x9, orig[3])) as u128;
		mixed <<= 8; mixed |= (ff_mult(0xe, orig[1]) ^ ff_mult(0xb, orig[2]) ^ ff_mult(0xd, orig[3]) ^ ff_mult(0x9, orig[0])) as u128;
		mixed <<= 8; mixed |= (ff_mult(0xe, orig[2]) ^ ff_mult(0xb, orig[3]) ^ ff_mult(0xd, orig[0]) ^ ff_mult(0x9, orig[1])) as u128;
		mixed <<= 8; mixed |= (ff_mult(0xe, orig[3]) ^ ff_mult(0xb, orig[0]) ^ ff_mult(0xd, orig[1]) ^ ff_mult(0x9, orig[2])) as u128;
	}
	mixed
}

fn ff_mult(mut a: u8, mut b: u8) -> u8 {
	let mut sum: u8 = 0;
	for _i in 0..8 {
		if b & 1 != 0 {
			sum ^= a;
		}
		b >>= 1;
		a = xtime(a);
	}
	sum
}

pub fn xtime(a: u8) -> u8 {
	if a & 0x80 != 0 {
		(a << 1) ^ 0x1b
	} else {
		a << 1
	}
}
