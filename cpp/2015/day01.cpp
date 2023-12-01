#include <iostream>
#include <fstream>

int main() {
  std::ifstream file;
  std::string str;
  int floor = 0;

  file.open("input/day01", std::ios::in);
  getline(file, str);

  for(char ch : str) {
    floor += ch == '(' ? 1 : -1;
  }

  std::cout << floor << '\n';

  return 0;
}