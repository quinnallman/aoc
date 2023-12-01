#include <iostream>
#include <fstream>

int main() {
  std::ifstream file;
  std::string str;
  int floor = 0;

  file.open("input/day01", std::ios::in);
  getline(file, str);

  for(int i = 0 ; i < str.length() ; i++) {
    floor += str[i] == '(' ? 1 : -1;
    if (floor < 0) {
      std::cout << i + 1 << '\n';
      break;
    }
  }

  return 0;
}