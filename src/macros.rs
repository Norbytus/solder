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
                $handler:expr,
                args: [
                    $([
                        $argument_name:expr,
                        $nullable:expr,
                        $variadic:expr,
                        $reference:expr
                    ]),* $(,)*
                ]
              ]),* $(,)*
        ]
    ) => {
        use zend::ModuleBuilder;

        #[no_mangle]
        pub extern fn info() {
            print_table_start();
            print_table_row($module_info, "enabled");
            print_table_end();
        }


        // $(php_function! {
        //     name: $function_name,
        //     args: [
        //         $([$argument_name, $nullable, $variadic, $reference])*
        //     ]
        // })*

        #[no_mangle]
        pub extern fn get_module() -> *mut zend::Module {
            let mut module = ModuleBuilder::new(
                c_str!($module_name),
                c_str!($module_version)
            ).with_info_function(info);

            $(
                let f: zend::Function = php_function! {
                    name: $function_name,
                    handler: $handler,
                    args: [
                        $([$argument_name, $nullable, $variadic, $reference],)*
                    ]
                };
                let module = module.with_function(f);
            )*;

            module.build().into_raw()
        }
    }
}

#[macro_export]
macro_rules! php_function {
    (
        name: $function_name:expr,
        handler: $handler:expr,
        args: [
            $([
                $name:expr,
                $nullable:expr,
                $variadic:expr,
                $reference:expr
              ]),* $(,)*
        ] $(,)*
    ) => ({

        let mut args: Vec<zend::ArgInfo> = Vec::new();

        $(
            args.push(zend::ArgInfo::new(
                c_str!($name),
                $nullable,
                $variadic,
                $reference
            ));
        )*;

        #[no_mangle]
        extern "C" fn wrap_php_function(
            data: &zend::ExecuteData,
            retval: &mut zend::Zval
        ) {
            let mut first_arg = zend::Zval::new_as_null();
            let mut second_arg = zend::Zval::new_as_null();

            php_parse_parameters!(&mut first_arg, &mut second_arg);

            let mut args: Vec<zend::Zval> = Vec::new();
            args.push(first_arg);
            args.push(second_arg);

            let result = $handler(args);

            php_return!(retval, result);
        }

        let mut function = zend::FunctionBuilder::new(
            c_str!($function_name),
            wrap_php_function
        );

        for arg in args {
            function = function.with_arg(arg);
        }


        function.build()
    })
}
