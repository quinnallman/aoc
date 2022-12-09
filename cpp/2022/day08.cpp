#include <iostream>
#include <vector>

using namespace std;

const int N = 99;

int grid[N][N];

bool is_visible(int row, int col) {
  int height = grid[row][col];

  int max = 0;
  for(int i = row - 1 ; i >= 0 ; i--) {
    if(grid[i][col] > max) {
      max = grid[i][col];
    }
  }
  if(height > max)
    return true;

  max = 0;
  for(int i = row + 1 ; i < N ; i++) {
    if(grid[i][col] > max) {
      max = grid[i][col];
    }
  }
  if(height > max)
    return true;

  max = 0;
  for(int i = col - 1 ; i >= 0 ; i--) {
    if(grid[row][i] > max) {
      max = grid[row][i];
    }
  }
  if(height > max)
    return true;

  max = 0;
  for(int i = col + 1 ; i < N ; i++) {
    if(grid[row][i] > max) {
      max = grid[row][i];
    }
  }
  if(height > max)
    return true;

  return false;
}

int main() {
  string line;

  int row = 0;
  while(getline(cin, line)) {
    for(int col = 0 ; col < N ; col++) {
      int tree_height = line[col] - '0';
      grid[row][col] = tree_height;
    }

    row++;
  }

  int num_visible = N * 2 + (N - 2) * 2;
  for(int i = 1 ; i < N - 1 ; i++) {
    for(int j = 1 ; j < N - 1 ; j++) {
      if(is_visible(i, j)) num_visible++;
    }
  }

  cout << num_visible << endl;

  return 0;
}