// Quadratic_equation
#[derive(Debug, PartialEq)]
pub enum Notice {
    VoSoNghiem,
    VoNghiem
}

#[derive(Debug, PartialEq)]
pub enum Value {
    OneSolution(f32),
    TwoSolution((f32, f32))
}

#[derive(Debug, PartialEq)]
pub enum Result {
    Value(Value),
    Notice(Notice)
}

pub fn quadratic_equation(a: i32, b: i32, c: i32) -> Result {
    if a == 0 {
        if b == 0 {
             if c == 0 {
                Result::Notice(Notice::VoSoNghiem)
             } else {
                Result::Notice(Notice::VoNghiem)
             }
        } else {
            let result = -c as f32 / b as f32;
            Result::Value(Value::OneSolution(result))
        }
    } else {
        let delta= b * b - 4 * a * c;
        if delta > 0 {
           let x1 = ((-b as f32) + (delta as f32).sqrt()) / (2 as f32 * a as f32);
           let x2 = ((-b as f32) - (delta as f32).sqrt()) / (2 as f32 * a as f32);
            Result::Value(Value::TwoSolution((x1, x2)))

        } else if delta == 0 {
            let x = -b as f32 / (2 as f32 * a as f32);
            Result::Value(Value::OneSolution(x))
        } else {
            Result::Notice(Notice::VoNghiem)
        }
    }
}


// Integer to Roman
pub fn int_to_roman(num: i16) -> String  {
    let symbol_list = [("I".to_string(), 1_i16), ("IV".to_string(), 4i16), ("V".to_string(), 5i16),
    ("IX".to_string(), 9i16), ("X".to_string(), 10i16), ("XL".to_string(), 40i16), ("L".to_string(), 50i16), ("XC".to_string(), 90i16), ("C".to_string(), 100i16), ("CD".to_string(), 400i16), ("D".to_string(), 500i16), ("CM".to_string(), 900i16), ("M".to_string(), 1000i16)];

    let mut res = String::from("");
    let mut number_integer = num;
    if  number_integer < 1 {
        return res
    }
    for (sym, val) in symbol_list.into_iter().rev() {
        if (number_integer as i16 / val) > 0 {
            let count = number_integer as usize / val as usize;
            res +=  sym.as_str().repeat(count).as_str();
            number_integer = number_integer % val;
        }
    }
    res
}

// Container most water
use std::cmp;
pub fn maxAreaWater(height: Vec<u8>) -> u32 {
    let mut res = 0_u32;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        // let a = cmp::min(height[left], height[right]);
        let area = (right - left) as u8 * cmp::min(height[left], height[right]);
        res = cmp::max(res, area as u32);

        if height[left] < height[right] {
            left += 1;
        }
        else {
            right -= 1;
        }
    }

    res
}