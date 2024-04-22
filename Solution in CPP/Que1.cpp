// Implement a function that checks whether a given string is a palindrome or not.

#include<iostream>
using namespace std;

bool checkPalindrome(string s){
    for(int i  = 0; i<s.length(); i++){
        if(s[i] != s[s.length() - 1 - i]){
            return false;
        }
    }

    return true;
}


int main(){
    string s1 = "nayan";

    int output1, output2;
    output1 = checkPalindrome(s1);

    if(output1 == 1){
        cout<<s1<<" is a palindrome"<<endl;
    }
    else{
        cout<<s1<<" is not a palindrome"<<endl;
    }


    return 0;


}
