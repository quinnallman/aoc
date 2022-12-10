#include <iostream>

using namespace std;

int get_signal(int cycles, int X) {
  if(cycles >= 20 && cycles <= 220 && ((cycles - 20) % 40 == 0)) {
    return cycles * X;
  }

  return 0;
}

int main() {
  int X = 1;
  int cycles = 0;
  int sum = 0;

  string line;
  while(getline(cin, line)) {
    string op = line.substr(0, line.find(' '));
    if(op == "addx") {
      int operand = stoi(line.substr(5));
      cycles++;
      sum += get_signal(cycles, X);
      cycles++;
      sum += get_signal(cycles, X);
      X += operand;
    } else {
      cycles++;
      sum += get_signal(cycles, X);
    }
  }

  cout << sum << endl;

  return 0;
}