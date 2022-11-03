
mod math;
use math::*;

fn main() {
    // let symbol_list = [("I", 1), ("IV", 4), ("V", 5),
    // ("IX", 9), ("X", 10), ("XL", 40), ("L", 50), ("XC", 90), ("C", 100), ("CD", 400), ("D", 500), ("CM", 900), ("M", 1000)];
   
    // for a in symbol_list {

    // }
}

#[cfg(test)]
mod test_quadratic_equation {
    use super::*;
    #[test]
    fn one_solution() {
        assert_eq!(quadratic_equation(0,1,2), Result::Value(Value::OneSolution(-2.0)))
    }

    #[test]
    fn two_solution() {
        assert_eq!(quadratic_equation(1,0,-4), Result::Value(Value::TwoSolution((2.0, -2.0))))
    }

    #[test]
    fn duplicate_solution() {
        assert_eq!(quadratic_equation(1, -4, 4), Result::Value(Value::OneSolution(2.0)))
    }

    #[test]
    fn no_solution_1() {
        assert_eq!(quadratic_equation(0, 0, 2), Result::Notice(Notice::VoNghiem))
    }

    #[test]
    fn no_solution_2() {
        assert_eq!(quadratic_equation(4, 1, 1), Result::Notice(Notice::VoNghiem))
    }

    #[test]
    fn infinity_solution() {
        assert_eq!(quadratic_equation(0, 0, 0), Result::Notice(Notice::VoSoNghiem))
    }

}

#[cfg(test)]
mod test_int_to_roman {
    use super::*;
    #[test]
    fn zero_number() {
        assert_eq!(int_to_roman(0), String::from(""))
    }

    #[test]
    fn negative_number() {
        assert_eq!(int_to_roman(-1), String::from(""))
    }

    #[test]
    fn valid_number_1() {
        assert_eq!(int_to_roman(3), String::from("III"))
    }

    
    #[test]
    fn valid_number_2() {
        assert_eq!(int_to_roman(1994), String::from("MCMXCIV"))
    }

    #[test]
    fn valid_number_3() {
        assert_ne!(int_to_roman(1994), String::from("III"))
    }
}


#[cfg(test)]
mod test_contaier_most_water {
    use super::*;
    #[test]
    fn case_one() {
        assert_eq!(maxAreaWater(vec![1,1]), 1)
    }

    #[test]
    fn case_two() {
        assert_eq!(maxAreaWater(vec![1,8,6,2,5,4,8,3,7]), 49)
    }

    #[test]
    fn case_three() {
        assert_ne!(maxAreaWater(vec![1,8,6,2,5,4,8,3,7]), 60)
    }
}


