// Check if a number is prime 
#include<iostream>
using namespace std;

bool isPrime(int number) {
    if(number <= 1) {
        return false; 
    }
    
    for(int i = 2; i*i<=number; ++i) {
        if(number%i == 0) {
            return false;
        }
    }


    return true;
}

int main() {
    int number = 29;

    if(isPrime(number)) {
        cout<<number<<" is a prime number"<<endl;
    } 
    else {
        cout<<number<<" is not a prime number"<<endl;
    }
    return 0;
}
