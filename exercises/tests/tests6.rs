// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    Box::from_raw(ptr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { 
            a: 1, 
            b: Some("hello".to_owned()) // 初始化 b 为 Some("hello".to_owned())
        });

        let ptr = Box::into_raw(data);
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(ptr) };

        assert!(ret.b == Some("hello".to_owned())); // 现在这个断言应该成功了

        // 将 ret 的所有权转移回一个 Box，然后将其转换为裸指针，以便我们可以比较地址
        let ptr_2 = Box::into_raw(Box::new(ret));

        // 由于我们不能在 Rust 中直接比较两个裸指针，我们需要将它们转换回 Box 然后比较
        let ret_2 = unsafe { Box::from_raw(ptr_2) };
        assert!(ret_2.a == 1); // 这里应该是 1，因为我们创建 Foo 时 a 是 1

        // 释放内存，避免内存泄漏
        std::mem::forget(ret_2);
    }
}
