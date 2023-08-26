fn main() {
    // ブロック1
    {
        let s1 = String::from("真実はワインの中にある");
        let s3 = String::from("ブドウ畑と美人は手が掛かる");
        // ブロック2
        {
            let s2 = s1;
            println!("{}", s2);
        }
        // s2はここで破棄される
        // これは所有権がs2へ移動されているので `value borrowed here after move` となる
        // println!("{}", s1);
        println!("{}", s3);
    }
    // s3の値はここで破棄される
}
