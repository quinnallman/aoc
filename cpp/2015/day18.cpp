#include <iostream>

using namespace std;

const int N = 100;
const int NUM_STEPS = 100;

bool get_light_state(bool board[][N], int row, int col) {
  int num_neighbours = 0;
  for(int y = row - 1 ; y <= row + 1 ; y++) {
    if(y < 0 || y >= N) continue;

    for(int x = col - 1 ; x <= col + 1 ; x++) {
      if(x < 0 || x >= N || (x == col && y == row)) continue;

      num_neighbours += board[y][x] ? 1 : 0;
    }
  }

  if(board[row][col]) return num_neighbours == 2 || num_neighbours == 3;
  else return num_neighbours == 3;

  return false;
}

void print_board(bool board[][N]) {
  for(int row = 0 ; row < N ; row++) {
    for(int col = 0 ; col < N ; col++) {
      cout << (board[row][col] ? '#' : '.');
    }
    cout << endl;
  }
  cout << endl << endl;
}

int main() {
  bool board[N][N];
  bool board2[N][N];
  string line;

  int row = 0;
  while(getline(cin, line)) {
    for(int col = 0 ; col < line.length() ; col++) {
      board[row][col] = line[col] == '#';
    }

    row++;
  }

  for(int step = 0 ; step < NUM_STEPS ; step++) {
    for(row = 0 ; row < N ; row++) {
      for(int col = 0 ; col < N ; col++) {
        board2[row][col] = get_light_state(board, row, col);
      }
    }

    for(row = 0 ; row < N ; row++) {
      for(int col = 0 ; col < N ; col++) {
        board[row][col] = board2[row][col];
      }
    }
  }

  int sum = 0;
  for(row = 0 ; row < N ; row++) {
    for(int col = 0 ; col < N ; col++) {
      sum += board[row][col] ? 1 : 0;
    }
  }

  cout << sum << endl;

  return 0;
}