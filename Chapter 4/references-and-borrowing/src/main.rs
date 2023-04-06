// mod undefined_behavior;
mod data_must_outlive_all_of_its_references;
mod dereferencing_a_pointer_accesses_its_data;
mod implicit_explicit_dereference;
mod more_convenient_greet;
mod mutable_references;
mod permissions_are_returned_at_the_end_of_a_reference_s_lifetime;
mod return_ownership_greet;
mod vec_example;

fn main() {
    // undefined_behavior::main();
    return_ownership_greet::main();
    more_convenient_greet::main();

    dereferencing_a_pointer_accesses_its_data::main();
    implicit_explicit_dereference::main();

    vec_example::main();
    mutable_references::main();

    permissions_are_returned_at_the_end_of_a_reference_s_lifetime::main();
}
