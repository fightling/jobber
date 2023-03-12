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

#[cfg(test)]
#[allow(dead_code)]
pub fn write(output: String) {
    unsafe {
        OUTPUT += &output;
    }
}

#[allow(dead_code)]
pub fn inspect() -> String {
    unsafe { OUTPUT.clone() }
}

#[cfg(test)]
#[macro_export]
macro_rules! outputln {
    () => {
        super::output::write(format!("\n"))
    };
    ($($arg:tt)*) => {{
        super::output::write(format!($($arg)*));
        super::output::write(format!("\n"))
    }};
}

#[cfg(test)]
#[macro_export]
macro_rules! output {
    () => ();
    ($($arg:tt)*) => {{
        super::output::write(format!($($arg)*));
    }};
}
