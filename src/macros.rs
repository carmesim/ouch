//! Macros used on ouch.

/// Macro that prints \[INFO\] messages, wraps [`println`].
///
/// There are essentially two different versions of the `info!()` macro:
/// - `info!(accessible, ...)` should only be used for short, important
///   information which is expected to be useful for e.g. blind users whose
///   text-to-speach systems read out every output line, which is why we
///   should reduce nonessential output to a minimum when running in
///   ACCESSIBLE mode
/// - `info!(inaccessible, ...)` can be used more carelessly / for less
///   important information. A seeing user can easily skim through more lines
///   of output, so e.g. reporting every single processed file can be helpful,
///   while it would generate long and hard to navigate text for blind people
///   who have to have each line of output read to them aloud, whithout to
///   ability to skip some lines deemed not important like a seeing person would.
///
/// By default `info` outputs to Stdout, if you want to specify the output you can use
/// `@display_handle` modifier

#[macro_export]
macro_rules! info {
    // Accessible (short/important) info message.
    // Show info message even in ACCESSIBLE mode
    (accessible, $($arg:tt)*) => {
        info!(@::std::io::stdout(), accessible, $($arg)*);
    };
    (@$display_handle: expr, accessible, $($arg:tt)*) => {
        let display_handle = &mut $display_handle;
        // if in ACCESSIBLE mode, suppress the "[INFO]" and just print the message
        if !(*$crate::cli::ACCESSIBLE.get().unwrap()) {
            $crate::macros::_info_helper(display_handle);
        }
        writeln!(display_handle, $($arg)*).unwrap();
    };
    // Inccessible (long/no important) info message.
    // Print info message if ACCESSIBLE is not turned on
    (inaccessible, $($arg:tt)*) => {
        info!(@::std::io::stdout(), inaccessible, $($arg)*);
    };
    (@$display_handle: expr, inaccessible, $($arg:tt)*) => {
        if (!$crate::cli::ACCESSIBLE.get().unwrap())
        {
            let display_handle = &mut $display_handle;
            $crate::macros::_info_helper(display_handle);
            writeln!(display_handle, $($arg)*).unwrap();
        }
    };
}

/// Helper to display "\[INFO\]", colored yellow
pub fn _info_helper(handle: &mut impl std::io::Write) {
    use crate::utils::colors::{RESET, YELLOW};

    write!(handle, "{}[INFO]{} ", *YELLOW, *RESET).unwrap();
}

/// Macro that prints \[WARNING\] messages, wraps [`eprintln`].
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        $crate::macros::_warning_helper();
        eprintln!($($arg)*);
    };
}

/// Helper to display "\[WARNING\]", colored orange
pub fn _warning_helper() {
    use crate::utils::colors::{ORANGE, RESET};

    if !crate::cli::ACCESSIBLE.get().unwrap() {
        print!("{}Warning:{} ", *ORANGE, *RESET);
    } else {
        print!("{}[WARNING]{} ", *ORANGE, *RESET);
    }
}
