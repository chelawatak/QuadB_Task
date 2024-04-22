// Given a sorted array of integers, implement a function that returns the median of the array.

#include<bits/stdc++.h>
using namespace std;

double findMedian(vector<int>& nums) {

    int n = nums.size();
    // If the number of elements is even

    if (n%2 == 0) { 
        int mid_right = n/2;
        int mid_left = mid_right - 1;
        return (nums[mid_left] + nums[mid_right])/2.0;

    // If the number of elements is odd
    } else { 
        int mid = n/2;
        return nums[mid];
    }
}

int main() {
    vector<int> sorted_array = {1, 2, 3, 4, 5,6};
    cout << "Median: " << findMedian(sorted_array) << endl; // Output: 3 (since 3 is in the middle)
    return 0;
}