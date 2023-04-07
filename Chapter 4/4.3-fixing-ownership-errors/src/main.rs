mod fixing_an_unsafe_program_not_enough_permissions;
mod fixing_an_unsafe_program_returning_a_reference_to_the_stack;

fn main() {
    fixing_an_unsafe_program_returning_a_reference_to_the_stack::return_a_string();
    fixing_an_unsafe_program_returning_a_reference_to_the_stack::return_a_string_literal();
    fixing_an_unsafe_program_returning_a_reference_to_the_stack::return_a_string_with_gc();
    let s: &mut String = &mut String::from("foo");
    fixing_an_unsafe_program_returning_a_reference_to_the_stack::return_a_string_mutable_reference(
        s,
    );
    println!("{}", s);

    ////////////////////////////////////////////////////////////////////////////////////////////////

    let name = &mut vec![String::from("Ferris"), String::from("Jr.")];
    fixing_an_unsafe_program_not_enough_permissions::stringify_name_with_title_mut(name);
    println!("Name is now {name:?}"); // modified! Unexpectedly!
    let name = vec![String::from("Ferris"), String::from("Jr.")];
    let full =
        fixing_an_unsafe_program_not_enough_permissions::stringify_name_with_title_take_ownership(
            name,
        );
    println!("Full is now {full}");
    let name = &vec![String::from("Ferris"), String::from("Jr.")];
    let full =
        fixing_an_unsafe_program_not_enough_permissions::stringify_name_with_title_clone(name);
    let full2 =
        fixing_an_unsafe_program_not_enough_permissions::stringify_name_with_title_without_clone(
            name,
        );
    println!("Finally, name={name:?} and full={full} and full2={full2}.")
}
