

use smallvec::{SmallVec, smallvec};

fn main() {
    let mut  v: SmallVec<i32, 4> = smallvec![1,2,3,4];

    println!("inline: {}", !v.spilled());

    v.push(5);

    println!("inline: {}", !v.spilled());

    v[0] = v[1] + v[2];

    println!("inline : {:?}", v);

    v.sort();

    println!("sorted: {:?}", v);

}
