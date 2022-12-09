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

int scenic_score(int row, int col) {
  int up = 0, down = 0, left = 0, right = 0;
  int num_trees = 0;
  for(int i = row - 1 ; i >= 0 ; i--) {
    num_trees++;
    if(grid[i][col] >= grid[row][col]) break;
  }
  up = num_trees;

  num_trees = 0;
  for(int i = row + 1 ; i < N ; i++) {
    num_trees++;
    if(grid[i][col] >= grid[row][col]) break;
  }
  down = num_trees;

  num_trees = 0;
  for(int i = col - 1 ; i >= 0 ; i--) {
    num_trees++;
    if(grid[row][i] >= grid[row][col]) break;
  }
  left = num_trees;

  num_trees = 0;
  for(int i = col + 1 ; i < N ; i++) {
    num_trees++;
    if(grid[row][i] >= grid[row][col]) break;
  }
  right = num_trees;

  //cout << up << " " << down << " " << left << " " << right << endl;
  return up * down * left * right;
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

  //cout << scenic_score(1, 2) << endl;
  int max = 0;
  for(int i = 1 ; i < N - 1 ; i++) {
    for(int j = 1 ; j < N - 1 ; j++) {
      int score = scenic_score(i, j);
      if(score > max) {
        max = score;
      }
    }
  }

  cout << max << endl;

  return 0;
}