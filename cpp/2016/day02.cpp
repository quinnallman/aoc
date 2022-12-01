#include <iostream>

using namespace std;

int main() {
  string line;
  pair<int, int> pos = make_pair(1, 1);
  int digits[][3] = {
    { 1, 2, 3 },
    { 4, 5, 6 },
    { 7, 8, 9 }
  };

  while(getline(cin, line)) {
    for(auto c : line) {
      if(c == 'U') {
        if(pos.first > 0) pos.first--;
      } else if(c == 'D') {
        if(pos.first < 2) pos.first++;
      } else if(c == 'L') {
        if(pos.second > 0) pos.second--;
      } else if(c == 'R') {
        if(pos.second < 2) pos.second++;
      }
    }
    cout << digits[pos.first][pos.second];
  }
  cout << endl;

  return 0;
}