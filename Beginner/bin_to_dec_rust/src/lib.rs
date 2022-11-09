#[cfg(test)]
mod tests {
    use crate::{binary_to_decimal, decimal_to_binary};

    #[test]
    fn binary_convert(){
        assert_eq!(binary_to_decimal("01001001"), 73);
        assert_eq!(binary_to_decimal("1100"), 12);
        assert_eq!(binary_to_decimal("101000000"), 320);
        assert_eq!(binary_to_decimal("1001100010011000111010000"), 20001232);

    }

    #[test]
    fn decimal_convert(){
        assert_eq!(decimal_to_binary("12"), "1100");
        assert_eq!(decimal_to_binary("320"), "101000000");
        assert_eq!(decimal_to_binary("20001232"), "1001100010011000111010000"); 
               
    }
}

pub fn binary_to_decimal(binary:&str) -> i64 {
    let mut result:u32 = 0;
    for (i,bit) in binary.chars().rev().enumerate() {
        let g=bit.to_digit(10).unwrap();
        result += g * u32::pow(2, i as u32);
    }
    result as i64
}

pub fn decimal_to_binary(decimal:&str) -> String {
    let mut result: String = String::new();   
    let mut division:i64=decimal.parse().unwrap();
    let mut module:i64=decimal.parse().unwrap();
    while  division >= 1 {
        module=division % 2;
        division = division / 2;
        result.insert_str(0, &module.to_string().to_owned());
    }
    result
}