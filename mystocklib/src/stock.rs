use std::convert::Into;

#[derive(Debug, Clone, PartialEq)]
struct Stock {
    name: String,
    value: f32,
}

impl Into<String> for Stock{
    fn into(self) -> String {
        self.name
    }
}

impl Into<f32> for Stock {
    fn into(self) -> f32 {
        self.value
    }

}

impl Stock {
    pub fn new<S: Into<String>>(name: S, value: f32) -> Stock {
        Stock {
            name: name.into(),
            value: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Stock;

    #[test]
    fn partial() {
        let stockA = Stock::new("ads", 43.43);
        let stockB = Stock::new(String::from("ads"), 43.43);

        assert_eq!(stockA, stockB);
    }

    #[test]
    fn clone() {
        let stockA = Stock::new("ads", 45.43);
        let stockB = stockA.clone();

        assert_eq!(stockA, stockB);
    }

    #[test]
    fn intostr() {
        let stockA = Stock::new("ads", 45.43);
        let name: String = stockA.into();

        assert_eq!(name, "ads".to_string());
    }

    #[test]
    fn intof32() {
        let stockA = Stock::new("ads", 45.43);
        let value: f32 = stockA.into();

        assert_eq!(value, 45.43);
    }
}
