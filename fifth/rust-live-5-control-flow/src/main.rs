/* 
 FIZZBUZZ PROBLEM 


*/


fn main(){
    for mut num in 1..100{
        if num % 3 == 0{
            print!("FIZZ");
            if num % 5 != 0{
                println!("");
            }
        };
        if num % 5 == 0{
            println!("BUZZ");
        };


        num += 1;
    }
}
