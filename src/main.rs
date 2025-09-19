use easyparse::*;

fn main() {
    println!("🎯 EasyParse: Parser Combinators Demo");
    println!("=====================================");
    
    // Test basic combinators
    println!("\n📝 Testing basic combinators:");
    
    // Test character parsing
    match char('a').parse("abc") {
        Ok((c, remaining)) => println!("✅ char('a').parse(\"abc\") = '{}', remaining: \"{}\"", c, remaining),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    // Test number parsing
    match number().parse("123abc") {
        Ok((num, remaining)) => println!("✅ number().parse(\"123abc\") = {}, remaining: \"{}\"", num, remaining),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    // Test string parsing
    match string("hello").parse("hello world") {
        Ok((s, remaining)) => println!("✅ string(\"hello\").parse(\"hello world\") = \"{}\", remaining: \"{}\"", s, remaining),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    println!("\n🧮 Testing toy language parser:");
    
    // Test expressions
    let test_cases = vec![
        "42",
        "3 + 4",
        "10 - 2",
        "5 * 6",
        "15 / 3",
        "2 + 3 * 4",
        "10 - 2 * 3",
        "(5 + 3) * 2",
        "x = 10",
    ];
    
    for expr in test_cases {
        match parse_and_evaluate(expr) {
            Ok((ast, value)) => println!("✅ \"{}\" → AST: {:?}, Value: {}", expr, ast, value),
            Err(e) => println!("❌ \"{}\" → Error: {}", expr, e),
        }
    }
    
    println!("\n🎉 Demo complete!");
}
