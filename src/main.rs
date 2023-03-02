use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]

// structure
struct Paragraph{
    name: String

}
#[derive(Serialize,Deserialize)]
struct  Article{
    article:String,
    author:String,
    paragraph:Vec<Paragraph>
}

fn main(){


    
}