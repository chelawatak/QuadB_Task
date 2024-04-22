// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

#include<bits/stdc++.h>
using namespace std;

int indexFinder(vector<int> arr, int num){
    for(int i= 0; i<arr.size(); i++){
        if(arr[i] == num){
            return i;
        }
    }

    return -1; // the number is not present in the array given.
}

int main(){
    vector<int> array = {1,2,2,3,4,5};
    int findOccurrenceOf = 2;
    
    // let index starting from 1. 
    cout<<"The first occurence of "<<findOccurrenceOf<<" is at index: "<<1 + indexFinder(array, findOccurrenceOf)<<endl;

    return 0;
}