pub mod interface {
    pub use core::fmt;

    // why create our own? 
    pub trait Write {
        /// write a single character.
        fn write_char(&self, c: char);

        /// Write a Rust format string.
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

        /// block until the last buffered character has been physically put on the TX wire.
        fn flush(&self);
    }

    pub trait Read {
        /// Read a single character.
        fn read_char(&self) -> char {
            ' ' 
        }

        // Clear RX Buffers, if any.
        fn clear_rx(&self);
    }

    pub trait Statistics {
        /// Return the number of characters written.
        fn chars_written(&self) -> usize {
            0
        }

        fn chars_read(&self) -> usize {
            0
        }
    }

    // Trait alias for full-fledged console.
    pub trait All = Write + Read + Statistics;
    
}
