use duchess::{java, prelude::*};

// Declare the java class that includes a native method
duchess::java_package! {
    package greeting;

    public class greeting.Native {
        private static native java.lang.String baseGreeting(java.lang.String);
    }
}

// Implement the native method with a Rust function
#[duchess::java_function(greeting.Native::baseGreeting)]
fn base_greeting(
    name: &java::lang::String,
) -> duchess::GlobalResult<String> {
    let name: String = name.to_rust().execute()?;
    println!("Hello {name}. I am a Rust function. How are you today?");
    Ok(format!("Hello, {name}"))
}
