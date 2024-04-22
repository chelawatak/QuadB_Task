// Given a string of words, implement a function that returns the shortest word in the string

#include<bits/stdc++.h>
using namespace std;

string solver(string str){
    string s;
	stringstream ss(str);
	vector<string> v;

	while (getline(ss, s, ' ')) {
        
        int sum = 0;
        for(int j = 0; j<s.length(); j++){
            sum += s[j];
        }

        if(sum != 0){
            v.push_back(s);
        }
		
	}
    
    string answer;
    int minLen = INT_MAX;
    for(int i = 0; i<v.size(); i++){
        if(minLen > v[i].length()){
            minLen = v[i].length();
            answer = v[i];
        }
    }
    return answer;
}

int main(){
    string s = "  Aniket       Chelawat is      solving the assignment   given     by     QuadB company  ";
    string shortestWord = solver(s);
    cout<<"Shorted word present in this string = "<<shortestWord<<endl;
    return 0;
}