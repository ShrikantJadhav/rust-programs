fn main() {

    let v = vec![1,2,3,4,5,6];

    println!("{:?}", v);

    // during every iteration,
    // value from v's index will get moved to p
    for p in v {
        println!("{:?}", p);
    }
    // You can not print v now as it was moved into p
    // and p was dropped at every iteration.
    //println!("{:?}", v)


    // Now lets consider a case where we partially
    // iterate over the vec

    let v1 = vec![1,2,3,4,5,6,7,8,9];
    let mut count = 1;
    for p in v1 {
        println!("{:?}", p);
        if count == 2 {
            break;
        }
        count+=1;
    }

    // we get the same error
    //println!("{:?}", v1);
    }
