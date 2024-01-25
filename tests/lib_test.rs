extern crate termite;

// Import the standard testing library
#[cfg(test)]
mod tests {
    // Import necessary items from the standard testing library
    #[test]
    fn it_works() {
        // Use functions from your library to test its behavior
        assert_eq!(termite::add(2, 2), 4);
    }
}