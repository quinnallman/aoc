#include <iostream>
#include <set>

using namespace std;

void add_elements(string s, set<string>& elements) {
  for(int i = 0 ; i < s.length() - 1;) {
    string element = s.substr(i, (i < s.length() - 1 && s[i+1] >= 'a' && s[i+1] <= 'z') ? 2 : 1);
    elements.insert(element);
    i += element.length();
  }
}

int count_elements(string s) {
  int count = 0;
  for(int i = 0 ; i < s.length() - 1;) {
    string element = s.substr(i, (i < s.length() - 1 && s[i+1] >= 'a' && s[i+1] <= 'z') ? 2 : 1);
    count++;
    i += element.length();
  }

  return count;
}

int main() {
  string line;
  set<string> elements;

  while(getline(cin, line)) {
    if(line.length() == 0) {
      break;
    }

    int loc = line.find(" => ");
    add_elements(line.substr(0, loc), elements);
    add_elements(line.substr(loc + 4), elements);
  }

  getline(cin, line);
  add_elements(line, elements);

  int num_elements = count_elements(line);
  int num_rn = 0, num_ar = 0, num_y = 0;
  for(int i = 0 ; i < line.length() - 1 ; i++) {
    if(line.substr(i, 2) == "Rn")
      num_rn++;
    else if(line.substr(i, 2) == "Ar")
      num_ar++;
    else if(line[i] == 'Y')
      num_y++;
  }

  cout << num_elements - num_rn - num_ar - 2*num_y - 1 << endl;

  return 0;
}