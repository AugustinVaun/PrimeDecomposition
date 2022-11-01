use std::vec;
// This function is used to generate a vector of all prime numbers leading to the limit set by the user

pub fn prime_generator (limit :i32 ) -> Vec<i32> {

    //Declaring the Vector until the limit set
    let mut primes : Vec<i32> = (2..limit+1).collect();
    
    //Declaring the iterators.
    let mut divisor = 0;
    let mut iterator :usize;
    let mut slider:usize;

    while divisor < primes.len() {
        iterator = divisor +1;
        while iterator < primes.len() {
            let result = primes[iterator] % primes[divisor];
        
            
            if result == 0 {
                slider = iterator;
                
                while slider < {primes.len()-1} {
                    primes[slider] = primes[slider+1];
                    slider +=1;
                }
                primes.pop();
            }
            iterator +=1;
        }
    divisor+=1;
    }
    return primes
}