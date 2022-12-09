#include <iostream>
#include <deque>
#include <map>

using namespace std;

int main() {
  string line;
  map<string, int> fs;
  deque<string> cwd;

  cwd.push_back("/");

  while(getline(cin, line)) {
    if(line[0] == '$') {
      string cmd = line.substr(2, line.find(' ', 2) - 2);
      if(cmd == "cd") {
        string dir = line.substr(5);
        if(dir == "/") {
          cwd.clear();
          cwd.push_back("/");
        } else if(dir == "..") {
          cwd.pop_back();
        } else {
          cwd.push_back(cwd.back() + dir + "/");
        }
      }
    } else {
      if(line.substr(0, 3) != "dir") {
        int size = stoi(line.substr(0, line.find(' ')));
        string d = cwd.back();
        while(true) {
          fs[d] += size;
          if(d == "/") break;
          d = d.substr(0, d.rfind('/', d.length() - 2) + 1);
        }
      }
    }
  }

  int sum = 0;
  for(auto c : fs) {
    if(c.second <= 100000)
      sum += c.second;
  }

  cout << sum << endl;
  return 0;
}