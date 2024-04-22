// Merge two sorted arrays in Rust


fn main() {
    let sorted_array1 = vec![1, 2, 3, 4, 5];
    let sorted_array2 = vec![2, 4, 6, 9];

    let mut ans = Vec::new();
    let (mut ind1, mut ind2) = (0, 0);
    while ind1 < sorted_array1.len() && ind2 < sorted_array2.len() {
        if sorted_array1[ind1] < sorted_array2[ind2] {
            ans.push(sorted_array1[ind1]);
            ind1 += 1;
        } else {
            ans.push(sorted_array2[ind2]);
            ind2 += 1;
        }
    }

    while ind1 < sorted_array1.len() {
        ans.push(sorted_array1[ind1]);
        ind1 += 1;
    }

    while ind2 < sorted_array2.len() {
        ans.push(sorted_array2[ind2]);
        ind2 += 1;
    }

    // Printing the answer
    for num in &ans {
        print!("{} ", num);
    }
}
