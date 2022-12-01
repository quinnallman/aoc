#include <iostream>
#include <vector>

using namespace std;

int main(int argc, char **argv) {
  vector<pair<int, int>> visited;
  char direction = 0;
  int distance = 0;
  int x = 0, y = 0;
  int facing = 0;
  bool found;

  visited.push_back(make_pair(x, y));

  while(!cin.eof()) {
    cin >> direction >> distance;
    cin.ignore(2);

    if(direction == 'L') {
      facing++;
      facing %= 4;
    } else {
      facing--;
      if(facing < 0) facing += 4;
    }

    while(distance-- > 0) {
      if(facing == 0) {
        y--;
      } else if(facing == 1) {
        x++;
      } else if(facing == 2) {
        y++;
      } else if(facing == 3) {
        x--;
      }

      found = false;
      for(auto p : visited) {
        if(p.first == x && p.second == y) {
          found=true;
          break;
        }
      }
      if(found) break;

      visited.push_back(make_pair(x, y));
    }

    if(found) break;
  }

  if(found) {
    //cout << "(" << x << ", " << y << ")" << endl;
    cout << abs(x+y) << endl;
  }

  return 0;
}