// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;
use std::time::Instant;

pub fn sum(v: Vec<u128>) -> u128 {

    let (left, right) = v.split_at(v.len() / 2);
    
    let (mut res1, mut res2): (u128, u128) = (0, 0);
    thread::scope( |scope| {
        scope.spawn( || {
            res1 = left.iter().sum()
        });
        
        scope.spawn ( || {
            res2 = right.iter().sum()
        });
    });
    
    
    res1 + res2
    
}

pub fn sum2(v: Vec<u128>) -> u128 {

    let (left, right) = v.split_at(v.len() / 2);
    let (left1, left2) = left.split_at(left.len() / 2);
    let (right1, right2) = right.split_at(right.len() / 2); 
    
    let (mut res1, mut res2, mut res3, mut res4): (u128, u128, u128, u128) = (0, 0, 0, 0);
    thread::scope( |scope| {
        scope.spawn( || {
            res1 = left1.iter().sum()
        });
        
        scope.spawn ( || {
            res2 = left2.iter().sum()
        });
        
         scope.spawn ( || {
            res3 = right1.iter().sum()
        });
        
         scope.spawn ( || {
            res4 = right2.iter().sum()
        });
    });
    
    
    res1 + res2 + res3 + res4
    
}

pub fn osum(v: Vec<u128>) -> u128 {
    v.iter().sum()
}
    
    
    


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
    
}
    fn main () {
        let iterations: u128 = 100;
        let mut t1 = 0;
        let mut t2 = 0;
        let mut t3 = 0;
        for _ in 0 .. iterations {
            let large_vector: Vec<u128> = (1..10000000).collect();
            
            let start_two = Instant::now();
            let two_sum = sum(large_vector.clone());
            t2 += start_two.elapsed().as_nanos();
            
            let start_one = Instant::now();
            let one_sum = osum(large_vector.clone());
            t1 += start_one.elapsed().as_nanos();
            
            let start_three = Instant::now();
            let three_sum = sum2(large_vector.clone());
            t3 += start_three.elapsed().as_nanos();
        
        }
       
        
        
        println!("Elapsed C1: {:?} ns" , t2/iterations);
        println!("Elapsed C2: {:?} ns", t3/iterations);
        println!("Elapsed R: {:?} ns", t1/iterations);
        
    }
