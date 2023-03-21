/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
    if n < 0 {
        return -1;
    }
    let mut count = 0;
    for i in 1..(n+1){
        count = count + i;
    }
    return count;
}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
    let mut count = 0;
    for i in ls {
        if i >= &s && i <= &e {
            count = count + 1;
        }
    }
    return count;
}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
    for i in target{
        if set.contains(i) == false{
            return false;
        }
    }
    return true;
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
    if ls.len() == 0 {
        return None;
    }
    let total = ls.iter().fold(0 as f64, |acc,x| acc + x);
    let number = ls.len() as f64;
    return Some(total/number);
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
    if ls.len() == 0 {
        return 0;
    }
    let mut base:i32 = 1;
    let mut total:i32 = 0;
    for x in ls.iter().rev(){
        total += x * base;
        base = base * 2;
    }
    return total;
}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut number = n;
    let mut divider = 2;
    while number > 1 {
        while number % divider == 0 {
            factors.push(divider);
            number = number / divider;
        }
        divider += 1;
        }
    return factors;
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();
    if lst.len() == 0 {
        return output;
    }
    else if lst.len () == 1 {
        output.push(lst[0]);
        return output;
    }
    else {
    let size = lst.len();
        for x in 1..size{
            output.push(lst[x]);
        }
        output.push(lst[0]);
    }
    return output;
}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {
    if target == "" {
        return true;
    }
    let length = target.len();
    let slength = s.len();
    if slength >= length {
        for x in 0..slength{
            if (slength - x) < length{
                return false;
            }
            if &s[x..(x+length)] == target {
                return true;
            }
        }
    }
    return false;

}

/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Option<&str> {
    if s.len() == 0 {
        return None;
    }
    let len = s.len();
    let mut count = 0;
    let mut ret_count = 0;
    let mut start = 0;
    let mut end = 0;
    let mut ret_start = 0;
    let mut ret_end = 0;
    let mut a = "";
    for x in 0..len{
      if a != &s[x..(x+1)]{
          a =  &s[x..(x+1)];
          ret_count = 1;
          ret_end = x;
          ret_start = x;
      }
      else {
          ret_count += 1;
          ret_end += 1;
          if ret_count > count {
              count = ret_count;
              end = ret_end;
              start = ret_start;
          }
      }
   }
  return Some(&s[start ..(end+1)]);
}
