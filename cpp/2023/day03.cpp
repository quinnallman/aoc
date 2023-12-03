#include <iostream>
#include <vector>
#include <unordered_map>
#include <algorithm>

int main() {
  std::string line;
  std::unordered_map<size_t, int> numbers;
  std::vector<std::vector<int>> grid;

  int y = 0;
  int next = 0;
  while(getline(std::cin, line)) {
    int current_number = 0;
    grid.push_back(std::vector<int>{});

    for(size_t x = 0 ; x < line.length() ; x++) {
      char ch = line[x];

      if(ch >= '0' && ch <= '9') {
        grid[y].push_back(next);
        current_number *= 10;
        current_number += ch - '0';
      } else if(ch == '.') {
        grid[y].push_back(-1);
        if(current_number > 0) {
          numbers[next++] = current_number;
          current_number = 0;
        }
      } else {
        grid[y].push_back(-2);
        if(current_number > 0) {
          numbers[next++] = current_number;
          current_number = 0;
        }
      }
    }

    if(current_number > 0) {
      numbers[next++] = current_number;
      current_number = 0;
    }
    y++;
  }

  int sum = 0;
  std::vector<size_t> seen;
  for(size_t y = 0 ; y < grid.size() ; y++) {
    for(size_t x = 0 ; x < grid[y].size() ; x++) {
      if(grid[y][x] == -2) {
        for(int i = -1 ; i <= 1 ; i++) {
          if(y+i < 0 || y+i >= grid.size()) continue;

          for(int j = -1 ; j <= 1 ; j++) {
            if(x+j < 0 || x+j >= grid[y].size()) continue;

            int index = grid[y+i][x+j];

            if(index >= 0 && std::find(seen.begin(), seen.end(), index) == std::end(seen)) {
              sum += numbers[index];
              seen.push_back(index);
            }
          }
        }
      }
    }
  }

  std::cout << sum << "\n";
  return 0;
}