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
  while(!cin.eof()) {
    string first, second, third;
    getline(cin, first);
    getline(cin, second);
    getline(cin, third);

    for(auto c : first) {
      if(second.find(c) != string::npos && third.find(c) != string::npos) {
        sum += get_priority(c);
        break;
      }
    }
  }

  cout << sum << endl;

  return 0;
}