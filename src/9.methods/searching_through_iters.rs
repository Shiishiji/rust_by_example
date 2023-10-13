
fn main() {
    // --------------
    // vectors
    // --------------
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // returns Some(2)
    println!("{:?}", vec1.iter().find(|&&x| x == 2));
    println!("{:?}", vec1.iter().find(|&&x| x == 2)); // Can be iterated over many times

    // returns position of 2 -> Some(1)
    println!("{:?}", vec1.iter().position(|&x| x == 2));

    println!("{:?}", vec1.into_iter().find(|&x| x == 2)); // Can't be iterated over many times (values are moved)

    // returns None
    println!("{:?}", vec2.iter().find(|&&x| x == 2));
    println!("{:?}", vec2.into_iter().find(|&x| x == 2));


    // --------------
    // arrays
    // --------------
    let arr1 = [1, 2, 3];
    let arr2 = [4, 5, 6];

    println!("{:?}", arr1.iter().find(|&&x| x == 2));
    println!("{:?}", arr2.iter().find(|&&x| x == 2));

    println!("{:?}", arr1.into_iter().find(|&x| x == 2));
    println!("{:?}", arr2.into_iter().find(|&x| x == 2));
}
