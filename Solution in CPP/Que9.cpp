// Reverse a string 
#include<bits/stdc++.h>
using namespace std;

string reverseString(string s){
    string result = "";
    for(int i = s.length() - 1; i>= 0; i--){
        result += s[i];
    }

    return result;
}

int main(){
    string s = "Aniket";
    string ans = reverseString(s);
    cout<<"Reverse = "<<ans<<endl;
}