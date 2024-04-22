// Implement a function that returns the kth smallest element in a given array.
// assuming k starts from 1 and k is always less than array size.
#include<bits/stdc++.h>
using namespace std;

int finder(vector<int> v, int k){
    map<int, int> mp;
    for(auto it: v){
        mp[it]++;
    }

    for(auto it: mp){
        while(it.second != 0){
            it.second --;
            k --;
            if(k == 0){
                return it.first;
            }
        }
    }

    return v[0];
}

int main(){
    vector<int> array = {7,5,3,1,1,2,6,8,11,2,6,7};
    int k = 2; // find the second smallest element

    // sorting the vector.
    int ans = finder(array, k);
    cout<<k<<"th smallest element in the given array is: "<<ans<<endl;



}