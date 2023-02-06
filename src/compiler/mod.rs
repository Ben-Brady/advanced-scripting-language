use crate::AST;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;

pub fn compile(ast: AST) -> String {
    let context = Context::create();
    let module = context.create_module("addition");
    let i32_type = context.i32_type();

    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let fn_val = module.add_function("add", fn_type, None);
    let entry_basic_block = context.append_basic_block(fn_val, "entry");

    let builder = context.create_builder();
    builder.position_at_end(entry_basic_block);

    let x = fn_val.get_nth_param(0).unwrap().into_int_value();
    let y = fn_val.get_nth_param(1).unwrap().into_int_value();

    let ret = builder.build_int_add(x, y, "add");
    let return_instruction = builder.build_return(Some(&ret));

    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    unsafe {
        type Addition = unsafe extern "C" fn(i32, i32) -> i32;
        let add: JitFunction<Addition> = execution_engine.get_function("add").unwrap();

        add.call(1, 2).to_string()
    }
}
