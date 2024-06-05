#[derive(Debug)]
pub struct Equation {
    x2coff: f64,
    x1coff: f64,
    x0coff: f64,
    order: u8,
}

impl Equation {
    fn set_order(x2: f64, x1: f64) -> u8 {
        if x2 != 0.0 {
            return 2;
        } else if x1 != 0.0 {
            return 1;
        }
        return 0;
    }
    fn quadratic(x2: f64, x1: f64, x0: f64) -> Self {
        Self {
            x2coff: x2,
            x1coff: x1,
            x0coff: x0,
            order: Self::set_order(x2, x1),
        }
    }

    fn copy(other: &Equation) -> Self {
        Self {
            x2coff: other.x2coff,
            x1coff: other.x1coff,
            x0coff: other.x0coff,
            order: Self::set_order(other.x2coff, other.x1coff),
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn order_second() {
		let x: Equation = Equation::quadratic(1.0, 1.0, 1.0);
		assert_eq!(x.order, 2);
	}
	
	#[test]
	fn order_first() {
		let x: Equation = Equation::quadratic(0.0, 1.0, 1.0);
		assert_eq!(x.order, 1);
	}
	
	#[test]
	fn order_zero() {
		let x: Equation = Equation::quadratic(0.0, 0.0, 1.0);
		assert_eq!(x.order, 0);
	}

}
