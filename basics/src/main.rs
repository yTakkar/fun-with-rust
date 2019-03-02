fn main() {
    let immutable = 10;
    println!("The value of immutable is {}", immutable);

    let mut mutable = 5;
    println!("The value of mutable is {}", mutable);
    mutable = 6;
    println!("The value of mutable is {}", mutable);

    // Boolean
    let b: bool = true;
    println!("Boolean: {}", b);   

    // Tuple
    let tup = (1,2,3);
    let (one, two, three) = tup;
    println!("Tuple: {}, {}, {}", one, two, three);

    // Array
    let array = [4,5,6];
    println!("Array: {}, {}, {}", array[0], array[1], array[2]);

    // Expressions
    let exp = {
        let x = 100;
        x + 1
    };
    println!("Expression: {}", exp);

    // Functions
    let c = some_function(4);
    println!("Function: {}", c);

    let area = area((10, 10));
    println!("Area: {}", area);

    // If else
    if_else();

    // Conditional value
    conditional_value();

    // Loops
    loop_();

    // For loop
    for_();

    // Struct
    let str = struct_();
    println!("Struct: (name: {}), (email: {})", str.name, str.email);

    // Enums
    enum_();
}

fn some_function(number: i32) -> i32 {
    println!("Function: The value of number is: {}", number);
    number
}

fn area(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

fn if_else() {
    let b: bool = true;
    if b {
        println!("If/else: 'b' is {}", b);
    } else {
        println!("If/else: 'b' is {}", b);
    }
}

fn conditional_value() {
    let value = if true {
        5
    } else {
        6
    };
    println!("Conditional_value: {}", value);
}

fn loop_() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
    println!("Loop: {}", result);
}

fn for_() {
    let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let mut total = 0;
    for element in arr.iter() {
        total += element;
    }
    println!("For loop: {}", total);
}

struct User {
    name: String,
    email: String
}

fn struct_() -> User {
    let user = User {
        name: String::from("Faiyaz"),
        email: String::from("www.shtakkar@gmail.com")
    };
    user
}

fn enum_() {
    #[derive(Debug)]
    enum Answer {
        YES,
        NO
    }

    struct SAnswer {
        question: String,
        answer: Answer
    }

    let answer_1 = SAnswer {
        question: String::from("Are youa a human?"),
        answer: Answer::YES
    };

    let answer_2 = SAnswer {
        question: String::from("Are youa a alien?"),
        answer: Answer::NO
    };

    println!("Enum: 1. Question: {}", answer_1.question);
    println!("Enum: 1. Answer: {:?}", answer_1.answer);

    println!("Enum: 2. Question: {}", answer_2.question);
    println!("Enum: 2. Answer: {:?}", answer_2.answer);
}
