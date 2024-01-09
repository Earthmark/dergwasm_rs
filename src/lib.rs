use std::fmt::Write;

pub mod dergwasm;

#[no_mangle]
fn do_thing() {
    let root = dergwasm::Slot::get_root_slot();
    writeln!(
        dergwasm::Console,
        "Root {:?} has name {:?}",
        root,
        root.get_name()
    )
    .unwrap();
    for child in root.children().iter() {
        writeln!(
            dergwasm::Console,
            "{:?} has name {:?}",
            child,
            child.get_name()
        )
        .unwrap();
    }
}

#[no_mangle]
fn print_test() {
    writeln!(dergwasm::Console, "Hello world!").unwrap();
}
