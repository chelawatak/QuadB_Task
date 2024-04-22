// Find the maximum subarray sum in Rust

#include<bits/stdc++.h>
using namespace std;

int main(){
    vector<int> array = {1,4,5,3,-8,1,5,7,-1,-2,8};
    
    int maxSubarraySum = INT_MIN;
    int maxSum = 0;
    for(int i= 0; i<array.size(); i++){
        maxSum += array[i];
        if(maxSum > maxSubarraySum){
            maxSubarraySum = maxSum;
        }

        if(maxSum <= 0){
            maxSum = 0;
        }
    }

    cout<<"Answer = "<<maxSubarraySum<<endl;
    return 0;
}