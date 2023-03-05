///! These macros are used to redirect output for testing purpose

#[cfg(not(test))]
#[macro_export]
macro_rules! outputln {
    () => {
        println!("")
    };
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}

#[cfg(not(test))]
#[macro_export]
macro_rules! output {
    () => ();
    ($($arg:tt)*) => {{
        print!($($arg)*);
    }};
}

static mut OUTPUT: String = String::new();

#[allow(dead_code)]
pub fn write(output: String) {
    unsafe {
        OUTPUT += &output;
    }
}

#[allow(dead_code)]
pub fn output() -> String {
    unsafe { OUTPUT.clone() }
}

#[cfg(test)]
#[macro_export]
macro_rules! outputln {
    () => {
        crate::output::write(format!("\n"))
    };
    ($($arg:tt)*) => {{
        crate::output::write(format!($($arg)*));
        crate::output::write(format!("\n"))
    }};
}

#[cfg(test)]
#[macro_export]
macro_rules! output {
    () => ();
    ($($arg:tt)*) => {{
        crate::output::write(format!($($arg)*));
    }};
}
