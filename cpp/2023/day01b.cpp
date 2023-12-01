#include <iostream>
#include <vector>

int main() {
  int sum = 0;
  std::string line;
  std::vector<std::string> numbers {
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
  };

  while(getline(std::cin, line)) {
    std::string first;
    std::string last;

    for(size_t i = 0 ; i < line.length() && first.length() < 1 ; i++) {
      if(line[i] >= '0' && line[i] <= '9') {
        first = line[i];
        break;
      } else {
        for(size_t j = 0 ; j < numbers.size() ; j++) {
          auto num = numbers[j];
          if(line.substr(i, num.length()) == num) {
            first = '0' + j + 1;
            break;
          }
        }
      }
    }

    for(int i = line.length() - 1 ; i >= 0 && last.length() < 1 ; i--) {
      if(line[i] >= '0' && line[i] <= '9') {
        last = line[i];
        break;
      } else {
        for(size_t j = 0 ; j < numbers.size() ; j++) {
          auto num = numbers[j];
          if(line.substr(i, num.length()) == num) {
            last = '0' + j + 1;
            break;
          }
        }
      }
    }

    sum += std::stoi("" + first + last);
  }

  std::cout << sum << '\n';

  return 0;
}