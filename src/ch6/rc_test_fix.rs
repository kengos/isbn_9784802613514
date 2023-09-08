use std::rc::Rc;

fn main() {
    // ヒープ領域にi32型の1000を設定し、ポインターをa_rcに設定
    let a_rc = Rc::new(1000);
    {
        // i32型への参照をb_rcにも作成
        let b_rc = Rc::clone(&a_rc);
        println!("{}", b_rc);
        // a_rcの参照カウンターを確認
        println!("参照数 = {}", Rc::strong_count(&a_rc));
    }
    // RC型なので利用可能
    println!("{}", a_rc);
    println!("参照数 = {}", Rc::strong_count(&a_rc));
}
