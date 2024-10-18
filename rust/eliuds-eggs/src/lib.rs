pub fn egg_count(display_value: u32) -> usize {
		let mut count = 0;
		let mut v = display_value;
		while v != 0 {
				v &= v - 1;
				count += 1;
		}
		count
}
