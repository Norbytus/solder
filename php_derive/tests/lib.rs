use php_derive::php_function;
use solder::zend::{ExecuteData, Zval};

#[test]
fn assign_attr() {

    #[php_function(test_function)]
    fn test_function(first_arg: String, second_arg: Option<i32>) {
    }

}
