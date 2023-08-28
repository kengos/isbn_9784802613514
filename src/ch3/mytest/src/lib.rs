#[derive(Debug, PartialEq)]
struct GItem {
    name: String,
    price: i64,
}

#[cfg(test)]
mod tests {
    // 外側の要素を利用(=> GItem)
    use super::*;

    #[test]
    fn calc_test1() {
        assert_eq!(100 * 2, 200);
        assert_eq!((1 + 2) * 3, 9);
        assert_eq!(1 + 2 * 3, 7);
    }

    #[test]
    fn calc_test2() {
        assert_eq!(2 * 3, 6);
        // わざと失敗
        // assert_eq!(2 * 3, 7);
    }

    #[test]
    fn array_test() {
        let a1 = [100, 200, 300];
        let a2 = [100, 200, 300];
        assert_eq!(a1, a2);
        let a3 = ["りんご".to_string(), "バナナ".to_string()];
        let a4 = [String::from("りんご"), String::from("バナナ")];
        assert_eq!(a3, a4);
    }

    #[test]
    fn vec_test() {
        let v1 = vec!["apple", "banana", "mango"];
        let mut v2: Vec<&str> = Vec::new();
        v2.push("apple");
        v2.push("banana");
        v2.push("mango");
        assert_eq!(v1, v2);
    }

    #[test]
    fn item_test() {
        let apple1 = GItem {
            name: String::from("りんご"),
            price: 2400,
        };
        let mut apple2 = GItem {
            name: "りんご".to_string(),
            price: 0,
        };
        apple2.price = 2400;
        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple1.price, apple2.price);
        // 構造全体を比較 #[derive(PartialEq)] が必要
        assert_eq!(apple1, apple2);
    }
}
