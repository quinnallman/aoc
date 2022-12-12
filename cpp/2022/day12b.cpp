#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

const int X = 113;
const int Y = 41;

bool can_go(pair<int, int> current, int direction, char elevation[Y][X]) {
  if(current.first < 0 || current.first >= Y || current.second < 0 || current.second >= X)
    return false;

  if(direction == 0 && current.second < (X - 1)) {
    return (elevation[current.first][current.second + 1] - elevation[current.first][current.second]) < 2;
  } else if(direction == 1 && current.first > 0) {
    return (elevation[current.first - 1][current.second] - elevation[current.first][current.second]) < 2;
  } else if(direction == 2 && current.second > 0) {
    return (elevation[current.first][current.second - 1] - elevation[current.first][current.second]) < 2;
  } else if(direction == 3 && current.first < (Y - 1)) {
    return (elevation[current.first + 1][current.second] - elevation[current.first][current.second]) < 2;
  }

  return false;
}

int main() {
  char elevation[Y][X];
  int row = 0;
  pair<int, int> end;
  int scores[Y][X];
  vector<pair<int, int>> starters;

  string line;
  while(getline(cin, line)) {
    for(int col = 0 ; col < X ; col++) {
      if(line[col] == 'S') {
        elevation[row][col] = 'a';
      } else if(line[col] == 'E') {
        elevation[row][col] = 'z';
        end = make_pair(row, col);
      } else {
        elevation[row][col] = line[col];
      }

      if(elevation[row][col] == 'a')
        starters.push_back(make_pair(row, col));
    }

    row++;
  }

  for(int i = 0 ; i < Y ; i++) {
    for(int j = 0 ; j < X ; j++) {
      if(i == end.first && j == end.second)
        scores[i][j] = 0;
      else
        scores[i][j] = -1;
    }
  }

  vector<pair<int, int>> visit;
  visit.push_back(make_pair(end.first, end.second));
  while(!visit.empty()) {
    pair<int, int> current = visit.back();
    visit.pop_back();

    int score = scores[current.first][current.second] + 1;
    if(can_go(make_pair(current.first, current.second + 1), 2, elevation)) {
      if(scores[current.first][current.second + 1] < 0 || scores[current.first][current.second + 1] > score) {
        scores[current.first][current.second + 1] = score;
        if(find(visit.begin(), visit.end(), make_pair(current.first, current.second + 1)) == visit.end())
          visit.push_back(make_pair(current.first, current.second + 1));
      }
    }
    if(can_go(make_pair(current.first - 1, current.second), 3, elevation)) {
      if(scores[current.first - 1][current.second] < 0 || scores[current.first - 1][current.second] > score) {
        scores[current.first - 1][current.second] = score;
        if(find(visit.begin(), visit.end(), make_pair(current.first - 1, current.second)) == visit.end())
          visit.push_back(make_pair(current.first - 1, current.second));
      }
    }
    if(can_go(make_pair(current.first, current.second - 1), 0, elevation)) {
      if(scores[current.first][current.second - 1] < 0 || scores[current.first][current.second - 1] > score) {
        scores[current.first][current.second - 1] = score;
        if(find(visit.begin(), visit.end(), make_pair(current.first, current.second - 1)) == visit.end())
          visit.push_back(make_pair(current.first, current.second - 1));
      }
    }
    if(can_go(make_pair(current.first + 1, current.second), 1, elevation)) {
      if(scores[current.first + 1][current.second] < 0 || scores[current.first + 1][current.second] > score) {
        scores[current.first + 1][current.second] = score;
        if(find(visit.begin(), visit.end(), make_pair(current.first + 1, current.second)) == visit.end())
          visit.push_back(make_pair(current.first + 1, current.second));
      }
    }
  }

  int min = 1000;
  for(auto s : starters) {
    if(scores[s.first][s.second] >= 0 && scores[s.first][s.second] < min)
      min = scores[s.first][s.second];
  }

  cout << min << endl;

  return 0;
}