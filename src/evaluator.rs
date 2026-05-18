use crate::parser::*;

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
    }
}
