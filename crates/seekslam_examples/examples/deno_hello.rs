#![allow(unused)]

//!  这个示例展示了如何在 Rust 中定义操作（ops），然后从 JavaScript 中调用它们。

use deno_cli::deno_core;
use deno_cli::deno_core::*;
use deno_cli::deno_error;

// 注意：
// 在这里我们反序列化为 `serde_json::Value`，但你可以
// 将其反序列化为任何实现了 `Deserialize` 特性的其他类型。

use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::v8;

fn main() {
  let mut runtime = JsRuntime::new(RuntimeOptions::default());

  // Evaluate some code
  let code = "let a = 1+4; a*2";
  let output: serde_json::Value =
    eval(&mut runtime, code).expect("Eval failed");

  println!("Output: {output:?}");

  let expected_output = serde_json::json!(10);
  assert_eq!(expected_output, output);
}

fn eval(
  context: &mut JsRuntime,
  code: &'static str,
) -> Result<serde_json::Value, String> {
  let res = context.execute_script("<anon>", code);
  match res {
    Ok(global) => {
      let scope = &mut context.handle_scope();
      let local = v8::Local::new(scope, global);
      // Deserialize a `v8` object into a Rust type using `serde_v8`,
      // in this case deserialize to a JSON `Value`.
      let deserialized_value =
        serde_v8::from_v8::<serde_json::Value>(scope, local);

      match deserialized_value {
        Ok(value) => Ok(value),
        Err(err) => Err(format!("Cannot deserialize value: {err:?}")),
      }
    }
    Err(err) => Err(format!("Evaling error: {err:?}")),
  }
}

// /// 一个用于对数字数组求和的操作。操作层会自动反序列化输入并序列化返回的 Result 和值。
// #[op2]
// fn op_sum(#[serde] nums: Vec<f64>) -> Result<f64, deno_error::JsErrorBox> {
//   // 对输入求和
//   let sum = nums.iter().fold(0.0, |a, v| a + v);
//   // 以 Result<f64, OpError> 的形式返回
//   Ok(sum)
// }

// fn main() {
//   // 构建一个提供自定义操作的 deno_core::Extension
//   const DECL: OpDecl = op_sum();
//   let ext = Extension {
//     name: "my_ext",
//     ops: std::borrow::Cow::Borrowed(&[DECL]),
//     ..Default::default()
//   };

//   // 初始化一个运行时实例
//   let mut runtime = JsRuntime::new(RuntimeOptions {
//     extensions: vec![ext],
//     ..Default::default()
//   });

//   // 现在我们来看看如何调用我们刚刚定义的操作。运行时自动包含一个 Deno.core 对象，
//   // 它有多个函数用于与之交互。你可以在 core.js 中找到它的定义。
//   runtime
//     .execute_script(
//       "<usage>",
//       r#"
// // 打印辅助函数，调用 Deno.core.print()
// function print(value) {
//   Deno.core.print(value.toString()+"\n");
// }

// const arr = [1, 2, 3];
// print("The sum of");
// print(arr);
// print("is");
// print(Deno.core.ops.op_sum(arr));

// // 错误的用法
// try {
//   print(Deno.core.ops.op_sum(0));
// } catch(e) {
//   print('Exception:');
//   print(e);
// }
// "#,
//     )
//     .unwrap();
// }
