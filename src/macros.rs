/// Adds a `\0` to the end of the string and gets its pointer
#[macro_export]
macro_rules! c_str {
    ($s:expr) => {{
        concat!($s, "\0").as_ptr() as *const i8
    }};
}

#[macro_export]
macro_rules! php_module {
    (
        name: $module_name:expr,
        version: $module_version:expr,
        info: $module_info:expr,
        functions: [
            $([
                $function_name:expr,
                args: [
                    $([
                        $argument_name:expr,
                        $nullable:expr,
                        $variadic:expr,
                        $reference:expr
                    ]),* $(,)*
                ]
              ]),* $(,)*
        ] $(,)*
    ) => {
        use super::zend::ModuleBuilder;

        #[no_mangle]
        pub extern fn info() {
            print_table_start();
            print_table_row(info);
            print_table_end();
        }

        let mut module = ModuleBuilder::new(c_str!(name), c_str!(version))
            .with_info_function(info);

        functions

        pub extern fn get_module() -> &mut zend::Module {
            module
        }
    }
}

#[macro_export]
macro_rules! php_function {
    (
        name: $function_name:expr,
        args: [
            $([
                $name:expr,
                $nullable:expr,
                $variadic:expr,
                $reference:expr
              ]),* $(,)*
        ] $(,)*
    ) => {

        $(
            zend::ArgInfo::new(
                c_str!($name),
                $nullable,
                $variadic,
                $reference
            );
        )*
    }
}
