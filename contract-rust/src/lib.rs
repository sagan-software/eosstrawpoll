// #![no_std]
#![feature(
    alloc,
    core_intrinsics,
    panic_implementation,
    lang_items,
    alloc_error_handler,
    proc_macro_non_items
)]

// extern crate alloc;
extern crate eosio_macros;
extern crate eosio_sys;
extern crate eosio_types;
// extern crate wee_alloc;

use eosio_macros::*;
use eosio_sys::prelude::*;

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
// // Need to provide a tiny `panic` implementation for `#![no_std]`.
// // This translates into an `unreachable` instruction that will
// // raise a `trap` the WebAssembly execution if we panic at runtime.
// #[panic_implementation]
// #[no_mangle]
// pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
//     unsafe {
//         ::core::intrinsics::abort();
//     }
// }

// // Need to provide an allocation error handler which just aborts
// // the execution with trap.
// #[alloc_error_handler]
// #[no_mangle]
// pub extern "C" fn oom(_: ::core::alloc::Layout) -> ! {
//     unsafe {
//         ::core::intrinsics::abort();
//     }
// }

#[no_mangle]
pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
    // let buf = &[0x41u8, 0x41u8, 0x42u8];

    // let s = CString::new("data data data dataaa!!!!").unwrap();
    let name = n!(williamcurry);

    match action {
        n!(hi) => unsafe { printn(n!(hiaction)) },
        n!(bye) => unsafe { printn(n!(byeaction)) },
        _ => unsafe { eosio_assert(0, cstr!("unknown action")) },
    };

    unsafe {
        // printi(-500);
        // printui128(p_imm);
        // printn(action);
        // printi(action_data_size() as i64);
        // require_auth(code);
        // printui(current_time());
        printn(name);
        printui(true as u64);
        printui(false as u64);
        // printi(111);
        // printi(222);
        // printi(333);
        prints(cstr!("hello world what is your name"));
        // printi(444);
        // eosio_assert(0, ptr);
        // prints("\nAccount creation time: \0".as_ptr());
        // printi(::eosiolib::permission::get_account_creation_time(code));
        // prints(s.as_ptr());
        // printi(::eosiolib::transaction::tapos_block_num() as i64);
        // let data_size = action_data_size();
        // printui(data_size as u64);
        // let mut bytes = vec![1u8; data_size as usize];
        // let mut bytes_slice = &mut bytes[..];
        // let c_string = CString::new(bytes).unwrap();
        // let bytes2 = c_string.as_bytes_with_nul();
        // read_action_data(bytes_slice.as_mut_ptr(), data_size);
        // let s = str::from_utf8_unchecked(bytes_slice);
        // prints(s.as_ptr());
        // printui(bytes_slice.len() as u64);
    }
}
