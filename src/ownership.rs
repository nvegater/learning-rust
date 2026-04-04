// --- Ownership & Borrowing ---
// In TS, every value is garbage-collected. In Rust, every value has ONE owner.
// When the owner goes out of scope, the value is dropped (freed).

/// Demonstrates "move" semantics.
/// Passing a String to this function *moves* ownership — the caller can't use it anymore.
/// (Like handing someone your laptop — you no longer have it.)
pub fn my_own_string_length(owned_string: String) -> usize {
    owned_string.len()
}

/// Demonstrates "borrowing".
/// The `&` means we borrow a reference — the caller keeps ownership.
/// (Like letting someone look at your laptop screen.)
pub fn borrowed_string_length(borrowed_reference_to_string: &String) -> usize { borrowed_reference_to_string.len() }


/// Demonstrates "mutable borrowing".
/// `&mut` lets us modify the value without taking ownership.
/// (Like letting someone type on your laptop — only one person at a time.)
pub fn append_world(mutable_string: &mut String) {
    mutable_string.push_str(", world!");
}

pub fn append_world_functional(s: String) -> String {
    s + ", world!"
}

/// Demonstrates that primitive types (i32, bool, f64, char) implement Copy.
/// They're cheap to copy, so Rust copies them instead of moving.
/// (Like TS numbers — assigning doesn't "move" anything.)
pub fn add_ten(copied_number: i32) -> i32 {
    copied_number + 10
}

// --- Slice borrowing ---
// A &str is a borrowed view into a String (or string literal).
// Think of it like a TS `readonly` view — you can read but not modify.

/// Takes a string slice — the most flexible way to accept string data.
/// Works with both &String (auto-coerced) and &str literals.
pub fn first_word(borrowed_string: &str) -> &str {
    // Find the first space, or return the whole string
    let first_space_index = borrowed_string.find(' ');
    match first_space_index {
        Some(pos) => &borrowed_string[..pos],
        None => borrowed_string,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_transfers_ownership() {
        let s = String::from("hello");
        let len = my_own_string_length(s);
        assert_eq!(len, 5);

        // This would NOT compile — s was moved into my_own_string_length:
        // println!("{}", s); // ERROR: value borrowed after move
        //
        // In TS terms: imagine `takeOwnership(s)` made `s` undefined afterwards.
        // Rust enforces this at COMPILE TIME, not runtime.
    }

    #[test]
    fn borrow_keeps_ownership() {
        let s = String::from("hello");
        let len = borrowed_string_length(&s); // & = borrow, not move
        assert_eq!(len, 5);

        // s is still valid here — we only lent a reference
        assert_eq!(s, "hello"); // This compiles fine!
        // But if I would remove this line, the compiler would clean the S.
        // Because the compiler knows that S is no longer used after it's been borrowed.
        // The difference is that Rust enforces this at COMPILE TIME, not runtime.
        // Javascript and other languages need to constantly check for memory leaks and clean them up.

    }

    #[test]
    fn mutable_borrow_modifies_in_place() {
        let mut s = String::from("hello"); // must be declared `mut`
        append_world(&mut s); // &mut = mutable borrow
        assert_eq!(s, "hello, world!");
        // This looks like more traditional JAvascript. I would prefer a more functional version:
    }
    #[test]
    fn mutable_borrow_modifies_in_place_functional() {
        let s = String::from("hello");
        let new_word = append_world_functional(s);
        assert_eq!(new_word, "hello, world!");

    }

    #[test]
    fn only_one_mutable_borrow_at_a_time() {
        let mut s = String::from("hello");

        // This is fine — one mutable borrow
        let r1 = &mut s;
        r1.push_str("!");
        // r1's borrow ends here (last use)

        // This is also fine — r1 is no longer active
        let r2 = &mut s;
        r2.push_str("!");

        assert_eq!(s, "hello!!");

        // But you CAN'T have two &mut alive at the same time:
        // let r1 = &mut s;
        // let r2 = &mut s; // ERROR: cannot borrow `s` as mutable more than once
        // r1.push_str("a");
    }

    #[test]
    fn copies_not_moves_for_primitives() {
        let x = 5;
        let y = add_ten(x);
        assert_eq!(y, 15);

        // x is still valid — i32 implements Copy (it's cheap, so Rust copies it)
        assert_eq!(x, 5);

        // String does NOT implement Copy (heap-allocated, could be huge),
        // so it moves instead. This is the key difference.
    }

    #[test]
    fn slices_borrow_part_of_data() {
        let s = String::from("hello world");
        let word = first_word(&s); // &String coerces to &str automatically
        assert_eq!(word, "hello");

        // Also works with string literals (which are already &str)
        let word2 = first_word("foo bar");
        assert_eq!(word2, "foo");

        // s is still valid
        assert_eq!(s, "hello world");
    }

    #[test]
    fn clone_to_keep_both() {
        // If you NEED two owners, explicitly .clone() (like structuredClone in TS)
        let s1 = String::from("hello");
        let s2 = s1.clone(); // deep copy — now both are independent
        let len = my_own_string_length(s1); // s1 is moved, but we still have s2
        assert_eq!(len, 5);
        assert_eq!(s2, "hello"); // s2 is fine
    }
}
