#include <iostream>
#include <sstream>
#include <vector>
#include <algorithm>

int main() {
  std::string str;

  int total_paper = 0;
  int total_ribbon = 0;

  while(getline(std::cin, str)) {
    std::istringstream iss(str);
    std::string dimension;
    std::vector<int> dimensions;
    while(getline(iss, dimension, 'x')) {
      dimensions.push_back(std::stoi(dimension));
    }

    std::sort(dimensions.begin(), dimensions.end());

    total_paper += 3 * dimensions[0] * dimensions[1] + 2 * dimensions[1] * dimensions[2] + 2 * dimensions[2] * dimensions[0];
    total_ribbon += 2 * dimensions[0] + 2 * dimensions[1] + dimensions[0] * dimensions[1] * dimensions[2];
  }

  std::cout << total_paper << '\n';
  std::cout << total_ribbon << '\n';

  return 0;
}