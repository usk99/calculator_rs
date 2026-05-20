use crate::parser::*;

/// AST を再帰的に評価して f64 の結果を返す
pub fn eval(expr: &Expr) -> Result<f64, String> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::BinOp { op, left, right } => {
            let l = eval(left)?;
            let r = eval(right)?;
            match op {
                Op::Plus => Ok(l + r),
                Op::Minus => Ok(l - r),
                Op::Star => Ok(l * r),
                Op::Slash => {
                    if r == 0.0 {
                        Err("Zero division exception".to_string())
                    } else {
                        Ok(l / r)
                    }
                }
            }
        }
        Expr::Call { name, args } => {
            let vals: Result<Vec<f64>, String> = args.iter().map(eval).collect();
            let vals = vals?;
            match name.as_str() {
                "sqrt" => {
                    if vals.len() == 2 {
                        Ok(vals[0].powf(1.0 / vals[1]))
                    } else {
                        Err("sqrt requires 2 arguments".to_string())
                    }
                }
                "pow" => {
                    if vals.len() == 2 {
                        Ok(vals[0].powf(vals[1]))
                    } else {
                        Err("pow requires 2 arguments".to_string())
                    }
                }
                "log10" => {
                    if vals.len() == 1 {
                        Ok(vals[0].log10())
                    } else {
                        Err("log10 requires 1 argument".to_string())
                    }
                }
                "log" => {
                    if vals.len() == 2 {
                        Ok(vals[0].log(vals[1]))
                    } else {
                        Err("log requires 2 arguments".to_string())
                    }
                }
                _ => Err(format!("unknown function: {}", name)),
            }
        }
    }
}
