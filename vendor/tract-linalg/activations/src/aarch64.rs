use crate::Op;
use std::io::Write;

pub fn translate(
    w: &mut impl Write,
    name: &str,
    suffix: &str,
    prog: &[Op<f32>],
) -> std::io::Result<()> {
    use Op::*;
    let mut constants = vec![];
    for op in prog {
        match op {
            LoadA(t) | LoadB(t) | LoadC(t) | AddConst(t) | SubConst(t) | MulConst(t)
            | MinConst(t) | MaxConst(t) | FMA(t) => constants.push(*t),
            _ => (),
        }
    }
    intro(w, name, suffix, "", &constants)?;
    let mut const_ix = 0;
    for op in prog {
        match op {
            MoveAB => move_a_b(w)?,
            MoveAC => move_a_c(w)?,
            MoveBA => move_b_a(w)?,
            MoveBC => move_b_c(w)?,
            MoveCA => move_c_a(w)?,
            MoveCB => move_c_b(w)?,
            LoadA(_) => {
                load_a(w, const_ix / 4 + 25, const_ix % 4)?;
                const_ix += 1
            }
            LoadB(_) => {
                load_b(w, const_ix / 4 + 25, const_ix % 4)?;
                const_ix += 1
            }
            LoadC(_) => {
                load_c(w, const_ix / 4 + 25, const_ix % 4)?;
                const_ix += 1
            }
            Abs => abs(w)?,
            Recip => recip(w)?,
            Add => a_op_assign_b(w, "add")?,
            Sub => a_op_assign_b(w, "sub")?,
            Mul => a_op_assign_b(w, "mul")?,
            Min => a_op_assign_b(w, "min")?,
            Max => a_op_assign_b(w, "max")?,
            AddConst(_) => {
                a_op_const(w, "add", const_ix / 4 + 25, const_ix % 4)?;
                const_ix += 1
            }
            SubConst(_) => {
                a_op_const(w, "sub", const_ix / 4 + 25, const_ix % 4)?;
                const_ix += 1
            }
            MulConst(_) => {
                a_op_const(w, "mul", const_ix / 4 + 25, const_ix % 4)?;
                const_ix += 1
            }
            MinConst(_) => {
                a_op_const(w, "min", const_ix / 4 + 25, const_ix % 4)?;
                const_ix += 1
            }
            MaxConst(_) => {
                a_op_const(w, "max", const_ix / 4 + 25, const_ix % 4)?;
                const_ix += 1
            }
            FMA(_) => {
                a_op_assign_b(w, "mul")?;
                a_op_const(w, "add", const_ix / 4 + 25, const_ix % 4)?;
                const_ix += 1
            }
            IfPosTE => if_pos_then_else(w)?,
            SwapBC => swap_b_c(w)?,
            Noop => (),
            _ => todo!(),
        }
    }
    outro(w, &constants)?;
    Ok(())
}

fn intro(
    w: &mut impl Write,
    name: &str,
    suffix: &str,
    g: &str,
    constants: &[f32],
) -> std::io::Result<()> {
    writeln!(
        w,
        "{}",
        format!(
            "
                            .text
                            .align 4

                            .cpu generic+fp+simd
                            .global {g}arm64simd_act_{name}_f32_32n_{suffix}
                            {g}arm64simd_act_{name}_f32_32n_{suffix}:

                            stp         d8, d9, [sp, #-16]!
                            stp         d10, d11, [sp, #-16]!
                            stp         d12, d13, [sp, #-16]!
                            stp         d14, d15, [sp, #-16]!

                            cmp         x1, 0
                            beq         .ok
                            "
        )
    )?;
    if constants.len() > 0 {
        writeln!(w, r"adr x2, .consts")?;
        let vectors = (constants.len() + 3) / 4;
        for i in 0..vectors {
            let v = i + 25;
            writeln!(w, "{}", format!("ld1 {{ v{v}.4s }}, [x2], 16"))?;
        }
    }

    writeln!(
        w,
        r"
           .outer_loop:
           ld1         {{v0.4s, v1.4s, v2.4s, v3.4s}}, [x0], 64
           ld1         {{v4.4s, v5.4s, v6.4s, v7.4s}}, [x0], 64
           sub         x0, x0, 128
           "
    )
}

fn outro(w: &mut impl Write, constants: &[f32]) -> std::io::Result<()> {
    writeln!(
        w,
        r#"
           .done:
           st1         {{v0.4s, v1.4s, v2.4s, v3.4s}}, [x0], 64
           st1         {{v4.4s, v5.4s, v6.4s, v7.4s}}, [x0], 64

           subs        x1, x1, 32
           bne         .outer_loop

           .ok:
           mov         x0, 0

           .return:
           ldp         d14, d15, [sp], #16
           ldp         d12, d13, [sp], #16
           ldp         d10, d11, [sp], #16
           ldp         d8, d9, [sp], #16
           ret

           .consts:
           "#
    )?;
    for c in constants {
        writeln!(w, ".float {c}")?;
    }
    let vectors = (constants.len() + 3) / 4;
    let needed = 4 * vectors;
    let padding = needed - constants.len();
    for _ in 0..padding {
        writeln!(w, ".float 0.0")?;
    }
    Ok(())
}

fn move_a_b(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(
        w,
        r#"
           and        v0.16b, v8.16b, v8.16b
           and        v1.16b, v9.16b, v9.16b
           and        v2.16b, v10.16b, v10.16b
           and        v3.16b, v11.16b, v11.16b
           and        v4.16b, v12.16b, v12.16b
           and        v5.16b, v13.16b, v13.16b
           and        v6.16b, v14.16b, v14.16b
           and        v7.16b, v15.16b, v15.16b
           "#
    )
}

fn move_a_c(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(
        w,
        r#"
           and        v0.16b, v16.16b, v16.16b
           and        v1.16b, v17.16b, v17.16b
           and        v2.16b, v18.16b, v18.16b
           and        v3.16b, v19.16b, v19.16b
           and        v4.16b, v20.16b, v20.16b
           and        v5.16b, v21.16b, v21.16b
           and        v6.16b, v22.16b, v22.16b
           and        v7.16b, v23.16b, v23.16b
           "#
    )
}

fn move_b_a(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(
        w,
        r#"
           and        v8.16b , v0.16b, v0.16b
           and        v9.16b , v1.16b, v1.16b
           and        v10.16b, v2.16b, v2.16b
           and        v11.16b, v3.16b, v3.16b
           and        v12.16b, v4.16b, v4.16b
           and        v13.16b, v5.16b, v5.16b
           and        v14.16b, v6.16b, v6.16b
           and        v15.16b, v7.16b, v7.16b
           "#
    )
}
fn move_b_c(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(
        w,
        r#"
           and        v8.16b , v16.16b, v16.16b
           and        v9.16b , v17.16b, v17.16b
           and        v10.16b, v18.16b, v18.16b
           and        v11.16b, v19.16b, v19.16b
           and        v12.16b, v20.16b, v20.16b
           and        v13.16b, v21.16b, v21.16b
           and        v14.16b, v22.16b, v22.16b
           and        v15.16b, v23.16b, v23.16b
           "#
    )
}
fn move_c_a(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(
        w,
        r#"
           and        v16.16b, v0.16b, v0.16b
           and        v17.16b, v1.16b, v1.16b
           and        v18.16b, v2.16b, v2.16b
           and        v19.16b, v3.16b, v3.16b
           and        v20.16b, v4.16b, v4.16b
           and        v21.16b, v5.16b, v5.16b
           and        v22.16b, v6.16b, v6.16b
           and        v23.16b, v7.16b, v7.16b
           "#
    )
}

fn move_c_b(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(
        w,
        r#"
           and        v16.16b, v8.16b , v8.16b
           and        v17.16b, v9.16b , v9.16b
           and        v18.16b, v10.16b, v10.16b
           and        v19.16b, v11.16b, v11.16b
           and        v20.16b, v12.16b, v12.16b
           and        v21.16b, v13.16b, v13.16b
           and        v22.16b, v14.16b, v14.16b
           and        v23.16b, v15.16b, v15.16b
           "#
    )
}

fn load_a(w: &mut impl Write, reg: usize, lane: usize) -> std::io::Result<()> {
    writeln!(
        w,
        "{}",
        format!(
            "
                            dup         v0.4s, v{reg}.s[{lane}]
                            dup         v1.4s, v{reg}.s[{lane}]
                            dup         v2.4s, v{reg}.s[{lane}]
                            dup         v3.4s, v{reg}.s[{lane}]
                            dup         v4.4s, v{reg}.s[{lane}]
                            dup         v5.4s, v{reg}.s[{lane}]
                            dup         v6.4s, v{reg}.s[{lane}]
                            dup         v7.4s, v{reg}.s[{lane}]
                            "
        )
    )
}

fn load_b(w: &mut impl Write, reg: usize, lane: usize) -> std::io::Result<()> {
    writeln!(
        w,
        "{}",
        format!(
            "
                            dup         v8.4s, v{reg}.s[{lane}]
                            dup         v9.4s, v{reg}.s[{lane}]
                            dup         v10.4s, v{reg}.s[{lane}]
                            dup         v11.4s, v{reg}.s[{lane}]
                            dup         v12.4s, v{reg}.s[{lane}]
                            dup         v13.4s, v{reg}.s[{lane}]
                            dup         v14.4s, v{reg}.s[{lane}]
                            dup         v15.4s, v{reg}.s[{lane}]
                            "
        )
    )
}

fn load_c(w: &mut impl Write, reg: usize, lane: usize) -> std::io::Result<()> {
    writeln!(
        w,
        "{}",
        format!(
            "
                            dup         v16.4s, v{reg}.s[{lane}]
                            dup         v17.4s, v{reg}.s[{lane}]
                            dup         v18.4s, v{reg}.s[{lane}]
                            dup         v19.4s, v{reg}.s[{lane}]
                            dup         v20.4s, v{reg}.s[{lane}]
                            dup         v21.4s, v{reg}.s[{lane}]
                            dup         v22.4s, v{reg}.s[{lane}]
                            dup         v23.4s, v{reg}.s[{lane}]
                            "
        )
    )
}

fn abs(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(
        w,
        r#"
           fabs        v0.4s, v0.4s
           fabs        v1.4s, v1.4s
           fabs        v2.4s, v2.4s
           fabs        v3.4s, v3.4s
           fabs        v4.4s, v4.4s
           fabs        v5.4s, v5.4s
           fabs        v6.4s, v6.4s
           fabs        v7.4s, v7.4s
           "#
    )
}

fn recip(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(
        w,
        r#"
           fmov        v24.4s, #1.0
           fdiv        v0.4s, v24.4s, v0.4s
           fdiv        v1.4s, v24.4s, v1.4s
           fdiv        v2.4s, v24.4s, v2.4s
           fdiv        v3.4s, v24.4s, v3.4s
           fdiv        v4.4s, v24.4s, v4.4s
           fdiv        v5.4s, v24.4s, v5.4s
           fdiv        v6.4s, v24.4s, v6.4s
           fdiv        v7.4s, v24.4s, v7.4s
           "#
    )
}

fn a_op_assign_b(w: &mut impl Write, op: &str) -> std::io::Result<()> {
    writeln!(
        w,
        "{}",
        format!(
            "
                            f{op}        v0.4s, v0.4s, v8.4s
                            f{op}        v1.4s, v1.4s, v9.4s
                            f{op}        v2.4s, v2.4s, v10.4s
                            f{op}        v3.4s, v3.4s, v11.4s
                            f{op}        v4.4s, v4.4s, v12.4s
                            f{op}        v5.4s, v5.4s, v13.4s
                            f{op}        v6.4s, v6.4s, v14.4s
                            f{op}        v7.4s, v7.4s, v15.4s
                            "
        )
    )
}

fn a_op_const(w: &mut impl Write, op: &str, reg: usize, lane: usize) -> std::io::Result<()> {
    writeln!(
        w,
        "{}",
        format!(
            "
                            dup          v24.4s, v{reg}.s[{lane}]
                            f{op}        v0.4s, v0.4s, v24.4s
                            f{op}        v1.4s, v1.4s, v24.4s
                            f{op}        v2.4s, v2.4s, v24.4s
                            f{op}        v3.4s, v3.4s, v24.4s
                            f{op}        v4.4s, v4.4s, v24.4s
                            f{op}        v5.4s, v5.4s, v24.4s
                            f{op}        v6.4s, v6.4s, v24.4s
                            f{op}        v7.4s, v7.4s, v24.4s
                            "
        )
    )
}

fn if_pos_then_else(w: &mut impl Write) -> std::io::Result<()> {
    writeln!(
        w,
        r#"
           fcmge       v0.4s, v0.4s, #0.0
           fcmge       v1.4s, v1.4s, #0.0
           fcmge       v2.4s, v2.4s, #0.0
           fcmge       v3.4s, v3.4s, #0.0
           fcmge       v4.4s, v4.4s, #0.0
           fcmge       v5.4s, v5.4s, #0.0
           fcmge       v6.4s, v6.4s, #0.0
           fcmge       v7.4s, v7.4s, #0.0
           bsl         v0.16b, v8.16b,  v16.16b
           bsl         v1.16b, v9.16b,  v17.16b
           bsl         v2.16b, v10.16b, v18.16b
           bsl         v3.16b, v11.16b, v19.16b
           bsl         v4.16b, v12.16b, v20.16b
           bsl         v5.16b, v13.16b, v21.16b
           bsl         v6.16b, v14.16b, v22.16b
           bsl         v7.16b, v15.16b, v23.16b
           "#
    )
}

fn swap_b_c(w: &mut impl Write) -> std::io::Result<()> {
    for base in 0..8 {
        let (b, c) = (base + 8, base + 16);
        writeln!(
            w,
            "{}",
            format!(
                "
                                and        v24.16b, v{b}.16b , v{b}.16b
                                and        v{b}.16b , v{c}.16b, v{c}.16b
                                and        v{c}.16b, v24.16b, v24.16b
                                "
            )
        )?;
    }
    Ok(())
}

//
// .floor:
//     b           .unsupported
//
// .two_pow_of_int:
//     b           .unsupported
//
// .noop:
//     b           .inner_loop
//
// .unsupported:
//     mov         x0, 1
//     b           .return
//
// .ok:
//     mov         x0, 0
//
