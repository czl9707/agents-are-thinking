pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

fn main() {
    let x = 34usize as isize;   // cast usize to isize
    let x = 10 as f32;      // isize to float
    let x = 10.45f64 as i8; // float to i8 (loses precision)
    let x = 4u8 as u64;     // gains precision
    let x = 400u16 as u8;   // 144, loses precision (and thus changes the value)
    println!("`400u16 as u8` gives {}", x);
    let x = -3i8 as u8;     // 253, signed to unsigned (changes sign)
    println!("`-3i8 as u8` gives {}", x);
    //let x = 45 as bool;  // FAILS! (use 45 != 0 instead)
    let x = true as usize;  // cast bool to usize (gives a 1)
}