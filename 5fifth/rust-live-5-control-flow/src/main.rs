/* 
 FIZZBUZZ PROBLEM 


*/


fn main(){
    for mut num in 1..100{
        if num % 3 == 0 && num % 5 == 0{
            println!("FIZZBUZZ");
        } else if num % 3 == 0{
            println!("FIZZ");
        } else  if num % 5 == 0{
            println!("BUZZ");
        }

        num += 1;
    }
}
