// Implement a function that checks whether a given number is prime or not.

#include<bits/stdc++.h>
using namespace std;

bool isPrime(int num){
    if(num <= 1){
        return false;
    }


    for(int i = 2; i*i<= num; i++){
        if(num%i == 0){
            return false;
        }
    }

    return true;
}

int main(){
    int n = 11;
    if(isPrime(n)){
        cout<<"Prime Number"<<endl;
    }

    else{
        cout<<"Non Prime Number"<<endl;
    }

    return 0;
}