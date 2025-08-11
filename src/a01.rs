use std::result;

pub fn a01() {
    // 1. Declaring Integer Variables:
    // Declaring
    let i8: i8;
    let i16: i16;
    let i32: i32;
    let i64: i64;
    let i128: i128;

    let u8: u8;
    let u16: u16;
    let u32: u32;
    let u64: u64;
    let u128: u128;

    // Assign
    i8 = -100;
    i16 = 25_000;
    i32 = 1_500_000;
    i64 = -9_000_000_000;
    i128 = 170_000_000_000_000;

    u8 = 200;
    u16 = 60_000;
    u32 = 2_000_000_000;
    u64 = 18_000_000_000_000;
    u128 = 300_000_000_000_000;

    // Printing
    println!("Part 1");
    println!("i8: {i8}");
    println!("i16: {i16}");
    println!("i32: {i32}");
    println!("i64: {i64}");
    println!("i128: {i128}");

    println!("u8: {}", u8);
    println!("u16: {}", u16);
    println!("u32: {}", u32);
    println!("u64: {}", u64);
    println!("u128: {}", u128);

    // 2. Conversions:
    println!("Part 2");
    let decimal_value = 56_333;
    println!("Hexadecimal value: {:#x}", decimal_value);

    let binary_value = format!("Binary value: {:#b}", decimal_value);
    println!("{}", binary_value);

    // 3. Exploring Limits:
    println!("Part 3");

    // i8 ---------------------------------------
    println!("i8");
    println!("MIN: {}", std::i8::MIN);
    println!("MAX: {}", std::i8::MAX);

    // println!("Overflow: {}", i8::MAX + 1);
    // println!("Underflow : {}", i8::MIN - 1);

    // i16 ---------------------------------------
    println!("i16");
    println!("MIN: {}", std::i16::MIN);
    println!("MAX: {}", std::i16::MAX);

    // println!("Overflow: {}", i16::MAX + 1);
    // println!("Underflow : {}", i16::MIN - 1);

    // i32 ---------------------------------------
    println!("i32");
    println!("MIN: {}", std::i32::MIN);
    println!("MAX: {}", std::i32::MAX);

    // println!("Overflow: {}", i32::MAX + 1);
    // println!("Underflow : {}", i32::MIN - 1);

    // i64 ---------------------------------------
    println!("i64");
    println!("MIN: {}", std::i64::MIN);
    println!("MAX: {}", std::i64::MAX);

    // println!("Overflow: {}", i64::MAX + 1);
    // println!("Underflow : {}", i64::MIN - 1);

    // i128 ---------------------------------------
    println!("i128");
    println!("MIN: {}", std::i128::MIN);
    println!("MAX: {}", std::i128::MAX);

    // println!("Overflow: {}", i128::MAX + 1);
    // println!("Underflow : {}", i128::MIN - 1);

    // u8 ---------------------------------------
    println!("u8");
    println!("MIN: {}", std::u8::MIN);
    println!("MAX: {}", std::u8::MAX);

    // println!("Overflow: {}", u8::MAX + 1);
    // println!("Underflow : {}", u8::MIN - 1);

    // u16 ---------------------------------------
    println!("u16");
    println!("MIN: {}", std::u16::MIN);
    println!("MAX: {}", std::u16::MAX);

    // println!("Overflow: {}", u16::MAX + 1);
    // println!("Underflow : {}", u16::MIN - 1);

    // u32 ---------------------------------------
    println!("u32");
    println!("MIN: {}", std::u32::MIN);
    println!("MAX: {}", std::u32::MAX);

    // println!("Overflow: {}", u32::MAX + 1);
    // println!("Underflow : {}", u32::MIN - 1);

    // u64 ---------------------------------------
    println!("u64");
    println!("MIN: {}", std::u64::MIN);
    println!("MAX: {}", std::u64::MAX);

    // println!("Overflow: {}", u64::MAX + 1);
    // println!("Underflow : {}", u64::MIN - 1);

    // u128 ---------------------------------------
    println!("u128");
    println!("MIN: {}", std::u128::MIN);
    println!("MAX: {}", std::u128::MAX);

    // println!("Overflow: {}", u128::MAX + 1);
    // println!("Underflow : {}", u128::MIN - 1);

    // 4. Integer Operations:
    println!("Part 4");

    // Aqui nada mais acontece, ao fazer o cast de i8 para i16 tudo ocorre normalmente
    let additional_value = (i8 as i16) + i16;
    println!("Additional value: {}", additional_value);

    // Aqui nada mais acontece, ao fazer o cast de i16 para i32 tudo ocorre normalmente
    let subtraction_value = (i16 as i32) - i32;
    println!("Subtraction value: {}", subtraction_value);

    // Aqui duas coisas acontecem, ao fazer i32 as i16 mudamos o valor de i32, pois ele corta os valores de bits, depois ocorre "attempt to multiply with overflow" pois o valor resultando passa de i16
    // let multiplication_value = i16 * (i32 as i16);
    // println!("Multiplication value: {}", multiplication_value);

    // Aqui também perdemos valor do i128 ao fazer o cast mas o resultado é 0 e dividindo por zero o resultado é 0
    let division_value = i64/(i128 as i64);
    println!("Division value: {}", division_value);
}