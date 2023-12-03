#include <iostream>
#include <boost/algorithm/string.hpp>

const int MAX_RED = 12;
const int MAX_GREEN = 13;
const int MAX_BLUE = 14;

int main() {
  std::string line;
  size_t sum = 0;

  while(getline(std::cin, line)) {
    size_t pos_colon = line.find(':');
    size_t game_number = std::stoi(line.substr(5, pos_colon - 5));

    line = line.substr(pos_colon + 2);
    std::vector<std::string> rounds;
    split(rounds, line, boost::is_any_of(";"));

    bool possible = true;

    for(auto& round : rounds) {
      boost::trim(round);

      std::vector<std::string> totals;
      split(totals, round, boost::is_any_of(","));

      for(auto& total : totals) {
        boost::trim(total);

        int count = 0;
        if(total.substr(total.length() - 3) == "red") {
          count = std::stoi(total.substr(0, total.length() - 4));
          if(count > MAX_RED) {
            possible = false;
            break;
          }
        } else if(total.substr(total.length() - 4) == "blue") {
          count = std::stoi(total.substr(0, total.length() - 5));
          if(count > MAX_BLUE) {
            possible = false;
            break;
          }
        } else if(total.substr(total.length() - 5) == "green") {
          count = std::stoi(total.substr(0, total.length() - 6));
          if(count > MAX_GREEN) {
            possible = false;
            break;
          }
        }
      }

      if(!possible)
        break;
    }

    if(possible)
      sum += game_number;
  }

  std::cout << sum << '\n';

  return 0;
}