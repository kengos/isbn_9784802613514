#[derive(Debug, Clone)]
pub enum Node {
    Nop,                                           // 何もしない
    Number(i64),                                   // 数値
    Calc(char, Box<Node>, Box<Node>),              // 計算式
    If(Box<Node>, Box<Vec<Node>>, Box<Vec<Node>>), // if文
    For(String, i64, i64, Box<Vec<Node>>),         // for文
    Print(Box<Node>),                              // print文(計算出力)
    PrintStr(String),                              // print文(定数出力)
    SetVar(String, Box<Node>),                     // 変数代入
    GetVar(String),                                // 変数参照
}

impl Node {
    pub fn calc(op: char, l: Node, r: Node) -> Node {
        Node::Calc(op, Box::new(l), Box::new(r))
    }

    pub fn if_(cond: Node, t: Vec<Node>, f: Vec<Node>) -> Node {
        Node::If(Box::new(cond), Box::new(t), Box::new(f))
    }
}
