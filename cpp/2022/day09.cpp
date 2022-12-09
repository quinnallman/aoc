#include <iostream>
#include <vector>

using namespace std;

int main() {
  string line;
  pair<int, int> head_pos = make_pair(0, 0);
  pair<int, int> tail_pos = make_pair(0, 0);
  vector<pair<int, int>> visited;

  visited.push_back(make_pair(0, 0));
  while(getline(cin, line)) {
    char dir = line[0];
    int n = stoi(line.substr(2));

    for(int i = 0 ; i < n ; i++) {
      if(dir == 'R') {
        head_pos.first++;
      } else if(dir == 'L') {
        head_pos.first--;
      } else if(dir == 'U') {
        head_pos.second--;
      } else if(dir == 'D') {
        head_pos.second++;
      }

      if(abs(head_pos.first - tail_pos.first) >= 2 || abs(head_pos.second - tail_pos.second) >= 2) {
        if(head_pos.first != tail_pos.first)
          tail_pos.first += head_pos.first > tail_pos.first ? 1 : -1;
        if(head_pos.second != tail_pos.second)
          tail_pos.second += head_pos.second > tail_pos.second ? 1 : -1;

        bool seen = false;
        for(auto v : visited) {
          if(v.first == tail_pos.first && v.second == tail_pos.second) {
            seen = true;
            break;
          }
        }
        if(!seen)
          visited.push_back(make_pair(tail_pos.first, tail_pos.second));
      }
    }

    // cout << line << endl;
    // cout << "H (" << head_pos.first << ", " << head_pos.second << ")" << endl;
    // cout << "T (" << tail_pos.first << ", " << tail_pos.second << ")" << endl;
  }

  cout << visited.size() << endl;

  return 0;
}