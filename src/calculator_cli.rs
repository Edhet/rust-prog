use std::io;

pub fn start() -> io::Result<()> {

    let operations = vec!["+", "-", "*", "/", "**", "sqrt", "cbrt", "exit"];
    let digit = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    println!(r"
    Welcome to the CAL-CLI.
    List of Operations:
        +, -, *, /, **, sqrt, cbrt.
        type 'exit' to quit.
    example expression: 
        19 sqrt.
        12 + 3.
    It does need spaces, any unecessary input will be ignored and wrong inputs return 0.
    ");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let divided_string: Vec<&str> = input.trim().split(" ").collect();

        let mut numbers = vec![];
        let mut operation = String::new();

        for string in divided_string {
            let mut number_buffer = String::new();

            if operations.contains(&string) {
                operation = string.to_string();
            }
            else {
                for char in string.chars() {
                    if digit.contains(&char) {
                        number_buffer.push(char);
                    }
                }
            }
            if number_buffer.len() != 0 {
                numbers.push(number_buffer.parse::<f64>().unwrap());
            }
        }
        
        if operation.as_str() == "exit" {
            break;
        }

        let result = compute(numbers, operation);

        println!("{}\n", result);
    }
    Ok(())
}

fn compute (numbers: Vec<f64>, operation: String) -> f64 {

    match operation.as_str() {
        "+" => return numbers[0] + numbers[1], 
        "-" => return numbers[0] - numbers[1], 
        "**" => return numbers[0] * numbers[1], 
        "/" =>  return numbers[0] / numbers[1], 
        "*" => return numbers[0] ** &numbers[1], 
        "sqrt" => return numbers[0].sqrt(),
        "cbrt" => return numbers[0].cbrt(),
        _ => return 0.0
    }
}