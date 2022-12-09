#include <iostream>

using namespace std;

int get_priority(char c) {
  if(c >= 'a' && c <= 'z') return c - 'a' + 1;
  if(c >= 'A' && c <= 'Z') return c - 'A' + 27;
  return 0;
}

int main() {
  string line;
  int sum = 0;
  while(getline(cin, line)) {
    string first = line.substr(0, line.length() / 2);
    string second = line.substr(line.length() / 2);

    for(auto c : first) {
      if(second.find(c) != string::npos) {
        cout << c << " " << get_priority(c) << endl;
        sum += get_priority(c);
        break;
      }
    }
  }

  cout << sum << endl;

  return 0;
}