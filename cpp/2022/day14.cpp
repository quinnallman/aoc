#include <iostream>
#include <set>
#include <deque>

using namespace std;

void add_point(int x, int y, set<pair<int, int>>& walls) {
  walls.insert(make_pair(x, y));
}

void add_range(int x1, int y1, int x2, int y2, set<pair<int, int>>& walls) {
  if(x1 != x2) {
    for(int i = (x1 < x2 ? x1 : x2) ; i <= (x2 > x1 ? x2 : x1) ; i++)
      add_point(i, y1, walls);
  } else if(y1 != y2) {
    for(int i = (y1 < y2 ? y1 : y2) ; i <= (y2 > y1 ? y2 : y1) ; i++)
      add_point(x1, i, walls);
  }
}

void get_corners(string line, deque<pair<int, int>>& corners) {
  while(true) {
    int pos = line.find(',');
    int x1 = stoi(line.substr(0, pos));
    int y1 = stoi(line.substr(pos + 1, line.find(' ') - pos - 1));
    corners.push_back(make_pair(x1, y1));

    pos = line.find(" -> ");
    if(pos == string::npos) return;

    line = line.substr(pos + 4);
  }
}

void print_cave(set<pair<int, int>>& walls, set<pair<int, int>>& sand, int left, int right, int bottom) {
  for(int y = 0 ; y <= bottom + 2 ; y++) {
    for(int x = 500 - bottom - 4; x <= 500 + bottom + 4; x++) {
      if(y == bottom + 2) cout << '#';
      else if(walls.contains(make_pair(x, y))) cout << '#';
      else if(sand.contains(make_pair(x, y))) cout << 'o';
      else if(x == 500 && y == 0) cout << '+';
      else cout << '.';
    }
    cout << endl;
  }
  cout << endl;
}

int main() {
  string line;
  set<pair<int, int>> walls;
  set<pair<int, int>> sand;
  int x1, y1, x2, y2;
  int right = 0, bottom = 0, left = 50000;
  int pos;

  while(getline(cin, line)) {
    deque<pair<int, int>> corners;
    get_corners(line, corners);

    x1 = corners.front().first;
    if(x1 < left) left = x1;
    if(x1 > right) right = x1;
    y1 = corners.front().second;
    if(y1 > bottom) bottom = y1;
    corners.pop_front();

    while(corners.size() > 0) {
      x2 = corners.front().first;
      if(x2 < left) left = x2;
      if(x2 > right) right = x2;
      y2 = corners.front().second;
      if(y2 > bottom) bottom = y2;
      corners.pop_front();

      add_range(x1, y1, x2, y2, walls);

      x1 = x2;
      y1 = y2;
    }
  }

  int x, y;
  int num_sand = 0;
  bool done = false;
  for(int step = 0 ; !done ; step++) {
    x = 500;
    y = 0;

    while(true) {
      if(y == bottom + 1) {
        sand.insert(make_pair(x, y));
        break;
      } else if(!walls.contains(make_pair(x, y+1)) && !sand.contains(make_pair(x, y+1))) {
        y++;
      } else if(!walls.contains(make_pair(x-1, y+1)) && !sand.contains(make_pair(x-1, y+1))) {
        x--;
        y++;
      } else if(!walls.contains(make_pair(x+1, y+1)) && !sand.contains(make_pair(x+1, y+1))) {
        x++;
        y++;
      } else {
        if(x == 500 && y == 0) done = true;
        sand.insert(make_pair(x, y));
        break;
      }
    }

    print_cave(walls, sand, left, right, bottom);

    if(sand.size() > num_sand) num_sand = sand.size();
    else break;
  }

  cout << sand.size() << endl;
  return 0;
}