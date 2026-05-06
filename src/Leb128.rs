pub mod leb128 {
	pub fn encode_u64(mut num: u64, buffer: &mut Vec<u8>) {
		if num == 0 {
			buffer.push(0);
			return;
		}

		while num != 0 {
			let mut byte = (num & 0b0111_1111) as u8;
			num >>= 7;
			if num != 0 {
				byte |= 0b1000_0000;
			}
			buffer.push(byte);
		}
	}

	pub fn encode_i64(mut num: i64, buffer: &mut Vec<u8>) {
		loop {
			let mut byte = (num & 0b0111_1111) as u8;
			num >>= 7; // rust guarantees padding with sign bit

			let sign_bit_set = (byte & 0b0100_0000) != 0;

			if (num == 0 && !sign_bit_set) || (num == -1 && sign_bit_set) {
				buffer.push(byte);
				break
			}

			byte |= 0b1000_0000;
			buffer.push(byte);
		}
	}
}

pub trait Leb128Encode {
	fn encode_leb128(self, buffer: &mut Vec<u8>);
}

impl Leb128Encode for u64 {
	fn encode_leb128(self, buffer: &mut Vec<u8>) {
		leb128::encode_u64(self, buffer)
	}
}
impl Leb128Encode for i64 {
	fn encode_leb128(self, buffer: &mut Vec<u8>) {
		leb128::encode_i64(self, buffer)
	}
}
