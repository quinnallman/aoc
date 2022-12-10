#include <iostream>

using namespace std;

void draw_pixel(int col, int X) {
  col = col % 40;
  if(abs(X - col) < 2)
    cout << '#';
  else
    cout << '.';
  if(col + 1 >= 40) {
    cout << endl;
  }
}

int main() {
  int X = 1;
  int cycles = 0;
  int col = 0;

  string line;
  while(getline(cin, line)) {
    string op = line.substr(0, line.find(' '));
    if(op == "addx") {
      int operand = stoi(line.substr(5));
      cycles++;
      draw_pixel(col++, X);
      cycles++;
      draw_pixel(col++, X);
      X += operand;
    } else {
      cycles++;
      draw_pixel(col++, X);
    }
  }

  return 0;
}