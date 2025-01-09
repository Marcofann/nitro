// Copyright 2021-2024, Offchain Labs, Inc.
// For license information, see https://github.com/OffchainLabs/nitro/blob/master/LICENSE

use crate::scenarios::{
    call, call_indirect, convert, data_type::DataType, global_get, global_set, if_op,
    instruction_with_1_arg_1_return, instruction_with_2_args_1_return, local_get, local_set,
    local_tee, select,
};
use clap::ValueEnum;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq, Debug)]
#[clap(rename_all = "PascalCase")]
pub enum Scenario {
    I32Add,
    I32And,
    I32Clz,
    I32Ctz,
    I32DivS,
    I32DivU,
    I32Eq,
    I32Eqz,
    I32GeS,
    I32GeU,
    I32GtU,
    I32GtS,
    I32LeU,
    I32LeS,
    I32LtU,
    I32LtS,
    I32Mul,
    I32Ne,
    I32Or,
    I32Popcnt,
    I32RemS,
    I32RemU,
    I32Rotl,
    I32Rotr,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Sub,
    I32WrapI64,
    I32Xor,
    I64ExtendI32S,
    Call,
    CallIndirect,
    GlobalGet,
    GlobalSet,
    If,
    LocalGet,
    LocalSet,
    LocalTee,
    Select,
}

trait ScenarioWatGenerator {
    fn write_specific_wat_beginning(&self, wat: &mut Vec<u8>);
    fn write_specific_exported_func_beginning(&self, wat: &mut Vec<u8>);
    fn write_wat_ops(&self, wat: &mut Vec<u8>, number_of_ops_per_loop_iteration: usize);
}

impl ScenarioWatGenerator for Scenario {
    fn write_specific_wat_beginning(&self, wat: &mut Vec<u8>) {
        match self {
            Scenario::Call => call::write_specific_wat_beginning(wat),
            Scenario::CallIndirect => call_indirect::write_specific_wat_beginning(wat),
            Scenario::GlobalGet => global_get::write_specific_wat_beginning(wat),
            Scenario::GlobalSet => global_set::write_specific_wat_beginning(wat),
            Scenario::I32Add => {}
            Scenario::I32And => {}
            Scenario::I32Clz => {}
            Scenario::I32Ctz => {}
            Scenario::I32DivS => {}
            Scenario::I32DivU => {}
            Scenario::I32Eq => {}
            Scenario::I32Eqz => {}
            Scenario::I32GeS => {}
            Scenario::I32GeU => {}
            Scenario::I32GtU => {}
            Scenario::I32GtS => {}
            Scenario::I32LeU => {}
            Scenario::I32LeS => {}
            Scenario::I32LtU => {}
            Scenario::I32LtS => {}
            Scenario::I32Mul => {}
            Scenario::I32Ne => {}
            Scenario::I32Or => {}
            Scenario::I32Popcnt => {}
            Scenario::I32RemS => {}
            Scenario::I32RemU => {}
            Scenario::I32Rotl => {}
            Scenario::I32Rotr => {}
            Scenario::I32Shl => {}
            Scenario::I32ShrS => {}
            Scenario::I32ShrU => {}
            Scenario::I32Sub => {}
            Scenario::I32WrapI64 => {}
            Scenario::I32Xor => {}
            Scenario::I64ExtendI32S => {}
            Scenario::If => {}
            Scenario::LocalGet => {}
            Scenario::LocalSet => {}
            Scenario::LocalTee => {}
            Scenario::Select => {}
        }
    }

    fn write_specific_exported_func_beginning(&self, wat: &mut Vec<u8>) {
        match self {
            Scenario::Call => {}
            Scenario::CallIndirect => {}
            Scenario::GlobalGet => {}
            Scenario::GlobalSet => {}
            Scenario::I32Add => {}
            Scenario::I32And => {}
            Scenario::I32Clz => {}
            Scenario::I32Ctz => {}
            Scenario::I32DivS => {}
            Scenario::I32DivU => {}
            Scenario::I32Eq => {}
            Scenario::I32Eqz => {}
            Scenario::I32GeS => {}
            Scenario::I32GeU => {}
            Scenario::I32GtU => {}
            Scenario::I32GtS => {}
            Scenario::I32LeU => {}
            Scenario::I32LeS => {}
            Scenario::I32LtU => {}
            Scenario::I32LtS => {}
            Scenario::I32Mul => {}
            Scenario::I32Ne => {}
            Scenario::I32Or => {}
            Scenario::I32Popcnt => {}
            Scenario::I32RemS => {}
            Scenario::I32RemU => {}
            Scenario::I32Rotl => {}
            Scenario::I32Rotr => {}
            Scenario::I32Shl => {}
            Scenario::I32ShrS => {}
            Scenario::I32ShrU => {}
            Scenario::I32Sub => {}
            Scenario::I32WrapI64 => {}
            Scenario::I32Xor => {}
            Scenario::I64ExtendI32S => {}
            Scenario::If => {}
            Scenario::LocalGet => local_get::write_specific_exported_func_beginning(wat),
            Scenario::LocalSet => local_set::write_specific_exported_func_beginning(wat),
            Scenario::LocalTee => local_tee::write_specific_exported_func_beginning(wat),
            Scenario::Select => {}
        }
    }

    fn write_wat_ops(&self, wat: &mut Vec<u8>, number_of_ops_per_loop_iteration: usize) {
        match self {
            Scenario::Call => call::write_wat_ops(wat, number_of_ops_per_loop_iteration),
            Scenario::CallIndirect => {
                call_indirect::write_wat_ops(wat, number_of_ops_per_loop_iteration)
            }
            Scenario::GlobalGet => global_get::write_wat_ops(wat, number_of_ops_per_loop_iteration),
            Scenario::GlobalSet => global_set::write_wat_ops(wat, number_of_ops_per_loop_iteration),
            Scenario::I32Add => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "add",
            ),
            Scenario::I32And => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "and",
            ),
            Scenario::I32Clz => instruction_with_1_arg_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "clz",
            ),
            Scenario::I32Ctz => instruction_with_1_arg_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "ctz",
            ),
            Scenario::I32DivS => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "div_s",
            ),
            Scenario::I32DivU => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "div_u",
            ),
            Scenario::I32Eq => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "eq",
            ),
            Scenario::I32Eqz => instruction_with_1_arg_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "eqz",
            ),
            Scenario::I32GeS => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "ge_s",
            ),
            Scenario::I32GeU => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "ge_u",
            ),
            Scenario::I32GtU => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "gt_u",
            ),
            Scenario::I32GtS => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "gt_s",
            ),
            Scenario::I32LeU => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "le_u",
            ),
            Scenario::I32LeS => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "le_s",
            ),
            Scenario::I32LtU => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "lt_u",
            ),
            Scenario::I32LtS => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "lt_s",
            ),
            Scenario::I32Mul => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "mul",
            ),
            Scenario::I32Ne => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "ne",
            ),
            Scenario::I32Or => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "or",
            ),
            Scenario::I32Popcnt => instruction_with_1_arg_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "popcnt",
            ),
            Scenario::I32RemS => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "rem_s",
            ),
            Scenario::I32RemU => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "rem_u",
            ),
            Scenario::I32Rotl => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "rotl",
            ),
            Scenario::I32Rotr => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "rotr",
            ),
            Scenario::I32Shl => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "shl",
            ),
            Scenario::I32ShrS => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "shr_s",
            ),
            Scenario::I32ShrU => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "shr_u",
            ),
            Scenario::I32Sub => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "sub",
            ),
            Scenario::I32WrapI64 => convert::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I64,
                DataType::I32,
                "wrap_i64",
            ),
            Scenario::I32Xor => instruction_with_2_args_1_return::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                "xor",
            ),
            Scenario::I64ExtendI32S => convert::write_wat_ops(
                wat,
                number_of_ops_per_loop_iteration,
                DataType::I32,
                DataType::I64,
                "extend_i32_s",
            ),
            Scenario::If => if_op::write_wat_ops(wat, number_of_ops_per_loop_iteration),
            Scenario::LocalGet => local_get::write_wat_ops(wat, number_of_ops_per_loop_iteration),
            Scenario::LocalSet => local_set::write_wat_ops(wat, number_of_ops_per_loop_iteration),
            Scenario::LocalTee => local_tee::write_wat_ops(wat, number_of_ops_per_loop_iteration),
            Scenario::Select => select::write_wat_ops(wat, number_of_ops_per_loop_iteration),
        }
    }
}

// Programs to be benchmarked have a loop in which several similar operations are executed.
// The number of operations per loop is chosen to be large enough so the overhead related to the loop is negligible,
// but not too large to avoid a big program size.
// Keeping a small program size is important to better use CPU cache, trying to keep the code in the cache.

fn write_common_wat_beginning(wat: &mut Vec<u8>) {
    wat.write_all(b"(module\n").unwrap();
    wat.write_all(b"    (import \"debug\" \"start_benchmark\" (func $start_benchmark))\n")
        .unwrap();
    wat.write_all(b"    (import \"debug\" \"end_benchmark\" (func $end_benchmark))\n")
        .unwrap();
    wat.write_all(b"    (memory (export \"memory\") 0 0)\n")
        .unwrap();
    wat.write_all(b"    (global $ops_counter (mut i32) (i32.const 0))\n")
        .unwrap();
}

fn write_exported_func_beginning(wat: &mut Vec<u8>) {
    wat.write_all(b"    (func (export \"user_entrypoint\") (param i32) (result i32)\n")
        .unwrap();
}

fn write_loop_beginning(wat: &mut Vec<u8>) {
    wat.write_all(b"        call $start_benchmark\n").unwrap();
    wat.write_all(b"        (loop $loop\n").unwrap();
}

fn write_wat_end(
    wat: &mut Vec<u8>,
    number_of_loop_iterations: usize,
    number_of_ops_per_loop_iteration: usize,
) {
    let number_of_ops = number_of_loop_iterations * number_of_ops_per_loop_iteration;

    // update ops_counter
    wat.write_all(b"            global.get $ops_counter\n")
        .unwrap();
    wat.write_all(
        format!(
            "            i32.const {}\n",
            number_of_ops_per_loop_iteration
        )
        .as_bytes(),
    )
    .unwrap();
    wat.write_all(b"            i32.add\n").unwrap();
    wat.write_all(b"            global.set $ops_counter\n")
        .unwrap();

    // check if we need to continue looping
    wat.write_all(b"            global.get $ops_counter\n")
        .unwrap();
    wat.write_all(format!("            i32.const {}\n", number_of_ops).as_bytes())
        .unwrap();
    wat.write_all(b"            i32.lt_s\n").unwrap();
    wat.write_all(b"            br_if $loop)\n").unwrap();

    wat.write_all(b"        call $end_benchmark\n").unwrap();

    wat.write_all(b"        i32.const 0)\n").unwrap();
    wat.write_all(b")").unwrap();
}

pub fn generate_wat(scenario: Scenario, output_wat_dir_path: Option<PathBuf>) -> Vec<u8> {
    let number_of_loop_iterations = 200_000;
    let number_of_ops_per_loop_iteration = 2000;

    let mut wat = Vec::new();

    write_common_wat_beginning(&mut wat);
    scenario.write_specific_wat_beginning(&mut wat);

    write_exported_func_beginning(&mut wat);
    scenario.write_specific_exported_func_beginning(&mut wat);
    write_loop_beginning(&mut wat);

    scenario.write_wat_ops(&mut wat, number_of_ops_per_loop_iteration);

    write_wat_end(
        &mut wat,
        number_of_loop_iterations,
        number_of_ops_per_loop_iteration,
    );

    // print wat to file if needed
    if let Some(output_wat_dir_path) = output_wat_dir_path {
        let mut output_wat_path = output_wat_dir_path;
        output_wat_path.push(format!("{:?}.wat", scenario));
        let mut file = File::create(output_wat_path).unwrap();
        file.write_all(&wat).unwrap();
    }

    wat
}
