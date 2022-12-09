#include <iostream>
#include <vector>

using namespace std;

pair<int, int> update_pos(pair<int, int> tail_pos, pair<int, int> head_pos) {
  pair<int, int> ret = make_pair(tail_pos.first, tail_pos.second);
  if(abs(head_pos.first - tail_pos.first) >= 2 || abs(head_pos.second - tail_pos.second) >= 2) {
    if(head_pos.first != tail_pos.first)
      ret.first += head_pos.first > tail_pos.first ? 1 : -1;
    if(head_pos.second != tail_pos.second)
      ret.second += head_pos.second > tail_pos.second ? 1 : -1;
  }

  return ret;
}

int main() {
  string line;
  vector<pair<int, int>> visited;
  pair<int, int> rope[10] = {
    make_pair(0, 0),
    make_pair(0, 0),
    make_pair(0, 0),
    make_pair(0, 0),
    make_pair(0, 0),
    make_pair(0, 0),
    make_pair(0, 0),
    make_pair(0, 0),
    make_pair(0, 0),
    make_pair(0, 0)
  };

  visited.push_back(make_pair(0, 0));
  while(getline(cin, line)) {
    char dir = line[0];
    int n = stoi(line.substr(2));

    for(int i = 0 ; i < n ; i++) {
      if(dir == 'R') {
        rope[0].first++;
      } else if(dir == 'L') {
        rope[0].first--;
      } else if(dir == 'U') {
        rope[0].second--;
      } else if(dir == 'D') {
        rope[0].second++;
      }

      for(int i = 1 ; i < 10 ; i++) {
        auto p = update_pos(rope[i], rope[i-1]);
        rope[i].first = p.first;
        rope[i].second = p.second;
      }

      bool seen = false;
      for(auto v : visited) {
        //if(v.first == rope[9].first && v.second == rope[9].second) {
        if(v == rope[9]) {
          seen = true;
          break;
        }
      }
      if(!seen)
        visited.push_back(make_pair(rope[9].first, rope[9].second));
    }

    // cout << line << endl;
    // cout << "H (" << head_pos.first << ", " << head_pos.second << ")" << endl;
    // cout << "T (" << tail_pos.first << ", " << tail_pos.second << ")" << endl;
  }

  cout << visited.size() << endl;

  return 0;
}