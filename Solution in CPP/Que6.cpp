#include <iostream>
#include <vector>
#include <string>

using namespace std;

string longestCommonPrefix(vector<string>& strs) {
    if(strs.empty()){
        return ""; // If the vector is empty, return an empty string
    } 

    for(int i = 0; i<strs[0].size();++i) {
        for(int j = 1;j<strs.size();++j) {
            if(i >= strs[j].size() || strs[j][i] != strs[0][i]) {
                return strs[0].substr(0, i);
            }
        }
    }
    // If no mismatch is found, return the first string
    return strs[0];
}

int main() {
    vector<string> strings={"flower", "flow", "flight"};
    cout<<"Longest common prefix: " << longestCommonPrefix(strings) << endl;
    return 0;
}
