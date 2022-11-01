use std::vec;
use std::io;
mod prime_generator;


fn main() {
    //Declaring main the variables
    let mut target: i32;
    let mut decomposition : Vec<i32> = Vec::new();
    decomposition.push(1);

    //Ask for the target and save it
    println!("Please input the number you want a prime decomposition of :");
    let mut written_target = String::new();
    io::stdin().read_line(&mut written_target)
    .expect("Failed to read line");
    
    target = written_target.trim().parse().unwrap();

    //Check for evenness of the number until impossible to divide by 2.
    while {target % 2} == 0 {
        target = target / 2;
        decomposition.push(2);
    }
    println!("{:?}", decomposition);

    //Create upper boundary to search from.
    let mut upper_boundary = {target+1}/2;
    
    //Iterate on the upper_boundary until it reaches 1
    while upper_boundary > 2 {
        //Create the prime vector up to the upper boundary
        let primes = prime_generator::prime_generator(upper_boundary);
        
        //create an iterator for the prime vector to go through from highest to lowest
        let mut iterator :usize = primes.len()-1;
        //Search from highest to lower prime for a divisor
        while {target % primes[iterator]} != 0 {
            iterator -=1;
        }

        //Add the divisor to the decomposition vector
        decomposition.push(primes[iterator]);
        //Change the new target to the quotient
        target = target/primes[iterator];
        //Redefine the boundary
        upper_boundary = {target+1}/2;
    }

    println!("The prime decomposition of {} is :", target);
    println!("{:?}", decomposition);
}