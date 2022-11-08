#[cfg(test)]
mod tests {
    use crate::binary_to_decimal;
    
    #[test]
    fn binary_convert(){
        let binary=String::from("01001001");
        assert_eq!(binary_to_decimal(&binary),73)
    }
}

pub fn binary_to_decimal(binary:&String) -> u64{
    let mut result:u32 = 0;
    for (i,bit) in binary.chars().rev().enumerate() {
        let g=bit.to_digit(10).unwrap();
        result += g * u32::pow(2, i as u32);
        println!("h{}",result);
    }
    result as u64
}