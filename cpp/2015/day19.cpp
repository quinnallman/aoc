#include <iostream>
#include <vector>
#include <map>
#include <algorithm>

using namespace std;

int main() {
  string line;
  map<string, vector<string>> replacements;

  while(getline(cin, line)) {
    if(line.length() == 0) {
      break;
    }

    int i_split = line.find(" => ");
    string lhs = line.substr(0, i_split);
    string rhs = line.substr(i_split + 4);

    replacements[lhs].push_back(rhs);
  }

  getline(cin, line);

  vector<string> new_molecules;
  for(int i = 0 ; i < line.length() ; i++) {
    int input_length = 1;
    if(i < line.length() - 1 && line[i+1] >= 'a' && line[i+1] <= 'z')
      input_length++;

    string character = line.substr(i, input_length);
    for(auto replacement : replacements[character]) {
      string molecule = line.substr(0, i) + replacement + line.substr(i+input_length);
      if(find(new_molecules.begin(), new_molecules.end(), molecule) == new_molecules.end()) {
        new_molecules.push_back(molecule);
      }
    }

    if(input_length > 1)
      i += input_length - 1;
  }

  cout << new_molecules.size() << endl;

  return 0;
}