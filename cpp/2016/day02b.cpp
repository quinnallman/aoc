#include <iostream>

using namespace std;

int main() {
  string line;
  pair<int, int> pos = make_pair(2, 0);
  char digits[][5] = {
    { 0,    0,    '1',  0,    0 },
    { 0,    '2',  '3',  '4',  0 },
    { '5',  '6',  '7',  '8',  '9' },
    { 0,    'A',  'B',  'C',  0 },
    { 0,    0,    'D',  0,    0 },
  };

  while(getline(cin, line)) {
    for(auto c : line) {
      if(c == 'U') {
        if(pos.first > 0 && digits[pos.first-1][pos.second] > 0) pos.first--;
      } else if(c == 'D') {
        if(pos.first < 4 && digits[pos.first+1][pos.second] > 0) pos.first++;
      } else if(c == 'L') {
        if(pos.second > 0 && digits[pos.first][pos.second-1] > 0) pos.second--;
      } else if(c == 'R') {
        if(pos.second < 4 && digits[pos.first][pos.second+1] > 0) pos.second++;
      }
    }
    cout << digits[pos.first][pos.second];
  }
  cout << endl;

  return 0;
}