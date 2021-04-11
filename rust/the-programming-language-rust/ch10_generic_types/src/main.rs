use ::generic_types::generic;
use ::generic_types::r#trait;

fn main() {
    generic::mixup_generic();
    r#trait::impl_trait_within_struct();
    r#trait::create_struct_with_default();
    let list = [1, 2, 3, 4, 5];
    let largest = r#trait::largest2(&list);
    println!("{}", largest);
}
