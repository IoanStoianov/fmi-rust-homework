

/// Десетична бройна система: 0-9
pub fn decimal(input: &str) -> Option<u32> {
    let x = digital_root(input, 10);
    match x {
        Ok(x) => Some(x),
        Err(_) => None
    }
}

// /// Шестнадесетична бройна система: 0-9, последвано от a-f
pub fn hex(input: &str) -> Option<u32> {
    let x = digital_root(input, 16);
    match x {
        Ok(x) => Some(x),
        Err(_) => None
    }
}

// /// Осмична бройна система: 0-7
pub fn octal(input: &str) -> Option<u32> {
    let x = digital_root(input, 8);
    match x {
        Ok(x) => Some(x),
        Err(_) => None
    }
}

// /// Двоична бройна система: 0-1
pub fn binary(input: &str) -> Option<u32> {
    let x = digital_root(input, 2);
    match x {
        Ok(x) => Some(x),
        Err(_) => None
    }
}

fn digital_root(input: &str, system: u32) -> Result<u32, std::io::Error>{
    let mut save: u32;
    let mut s: String = input.to_string();
    while s.chars().count() > 1 {
        save = 0;
        for c in s.chars() {
            let digit = char_to_num(c);
            println!("{}",digit);
            save = save + digit;
        }
        s = match system {
            2 => format!("{:b}",save),
            8 => format!("{:o}",save),
            10 => format!("{}",save),
            16 => format!("{:x}",save),
            _ => panic!("Bad system"),
        };
        println!("{}",s);
    }

    let x = s.chars().next().unwrap();
    Ok(char_to_num(x))
}

fn char_to_num(c: char) -> u32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => panic!("Bad char"),
    }
}


#[test]
fn test_basic() {
    assert_eq!(decimal("a345"), Some(3));
    assert_eq!(decimal("345"), Some(3));
    assert_eq!(hex("345"), Some(0xc));

    assert_eq!(octal("1"), Some(1));
    assert_eq!(binary("1"), Some(1));

    let num = String::from("1");
    assert_eq!(binary(&num[..]), Some(1));
}
