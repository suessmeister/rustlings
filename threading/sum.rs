pub fn sum2(v: Vec<u64>) -> u64 {

    let (left, right) = v.split_at(v.len() / 2);
    let (left1, left2) = v.split_at(left.len() / 2);
    let (right1, right2) = v.split_at(right.len() / 2); 
    
    let (mut res1, mut res2, mut res3, mut res4): (u64, u64, u64, u64) = (0, 0, 0, 0);
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