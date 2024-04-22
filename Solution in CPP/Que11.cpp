// Merge two sorted arrays 

#include<bits/stdc++.h>
using namespace std;

int main(){
    vector<int>sorted_array1 = {1,2,3,4,5};
    vector<int>sorted_array2 = {2,4,6,9};

    vector<int> ans;
    int ind1 = 0, ind2 = 0;
    while(ind1 < sorted_array1.size() && ind2 < sorted_array2.size()){
        if(sorted_array1[ind1] < sorted_array2[ind2]){
            ans.push_back(sorted_array1[ind1]);
            ind1++;
        }

        else{
            ans.push_back(sorted_array2[ind2]);
            ind2++;
        }
    }

    while(ind1 < sorted_array1.size()){
        ans.push_back(sorted_array1[ind1]);
        ind1 ++;
    }

    while(ind2 < sorted_array2.size()){
        ans.push_back(sorted_array2[ind2]);
        ind2 ++;
    }



    // printing the answer;
    for(int i= 0; i<ans.size(); i++){
        cout<<ans[i]<<" ";
    }

    return 0;
}
