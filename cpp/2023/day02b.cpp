#include <iostream>
#include <boost/algorithm/string.hpp>

int main() {
  std::string line;
  size_t sum = 0;

  while(getline(std::cin, line)) {
    size_t pos_colon = line.find(':');

    line = line.substr(pos_colon + 2);
    std::vector<std::string> rounds;
    split(rounds, line, boost::is_any_of(";"));

    int min_red = 0, min_green = 0, min_blue = 0;

    for(auto& round : rounds) {
      boost::trim(round);

      std::vector<std::string> totals;
      split(totals, round, boost::is_any_of(","));

      for(auto& total : totals) {
        boost::trim(total);

        int count = 0;
        if(total.substr(total.length() - 3) == "red") {
          count = std::stoi(total.substr(0, total.length() - 4));
          if(count > min_red) {
            min_red = count;
          }
        } else if(total.substr(total.length() - 5) == "green") {
          count = std::stoi(total.substr(0, total.length() - 6));
          if(count > min_green) {
            min_green = count;
          }
        } else if(total.substr(total.length() - 4) == "blue") {
          count = std::stoi(total.substr(0, total.length() - 5));
          if(count > min_blue) {
            min_blue = count;
          }
        }
      }
    }

    sum += min_red * min_green * min_blue;
  }

  std::cout << sum << '\n';

  return 0;
}