
// binary search
fn bs_list(haystack: Vec<i32>, needle: i32) -> bool {
    let mut lo: i32 = 0;
    let mut hi: i32 = haystack.len().try_into().unwrap();

    while lo < hi {
        let m = (lo + (hi - lo) / 2).try_into().unwrap();
        let v = haystack[m as usize];

        if v == needle {
            return true
        } 
        else if v > needle {
            hi = m;
        } else {
            lo = m + 1;
        }
    }

    return false

}

// bubble sort
fn bubble_sort(mut list: Vec<i32>) -> Vec<i32> {
    for i in 0..list.len().try_into().unwrap() {
        for j in 0..list.len() - 1 - i {
            if list[j] > list[j + 1] {
                let temp = list[j];
                list[j] = list[j + 1];
                list[j + 1] = temp
            }
        }
    }

    return list;
}


fn main() {
    let mut haystack = Vec::new();
    haystack.extend([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
    let needle_in_stack = bs_list(haystack, 12);

    let mut list_to_sort = Vec::new();
    list_to_sort.extend([22, 13, 1, 6, 19, 30, 44]);
    let sorted = bubble_sort(list_to_sort);
    let mut right_anwser = Vec::new();
    right_anwser.extend([1, 6, 13, 19, 22, 30, 44]);

    let sorted = sorted == right_anwser;

    println!("needle in haystack is: {}", needle_in_stack);
    println!("list_to_sort got sorted correctly: {}", sorted);
}