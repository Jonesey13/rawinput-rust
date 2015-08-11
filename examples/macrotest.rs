

macro_rules! vec_mrk2{( $ elem : expr ; $ n : expr ) => ( $ crate:: vec:: from_elem ( $ elem , $ n )
) ; ( $ ( $ x : expr ) , * ) => (
< [ _ ] > :: into_vec ( Box:: new ( [ $ ( $ x ) , * ] ) ) )
; ( $ ( $ x : expr , ) * ) => ( vec_mrk2 ! [ $ ( $ x ) , * ] )
}

fn main(){
let a = vec_mrk2!(1,2,);
println!("{:?}",a);
}
