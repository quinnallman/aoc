#include <iostream>
#include <map>

int main() {
  auto current = std::make_pair(0, 0);
  auto robo_current = std::make_pair(0, 0);
  bool is_robo = false;

  std::map<std::pair<int, int>, int> deliveries;

  deliveries[current] = 1;

  std::string line;

  while(getline(std::cin, line)) {
    for(auto ch : line) {
      switch(ch) {
        case '^':
          if (is_robo)
            robo_current = std::make_pair(robo_current.first, robo_current.second - 1);
          else
            current = std::make_pair(current.first, current.second - 1);
          break;
        case 'v':
          if (is_robo)
            robo_current = std::make_pair(robo_current.first, robo_current.second + 1);
          else
            current = std::make_pair(current.first, current.second + 1);
          break;
        case '<':
          if (is_robo)
            robo_current = std::make_pair(robo_current.first - 1, robo_current.second);
          else
            current = std::make_pair(current.first - 1, current.second);
          break;
        case '>':
          if (is_robo)
            robo_current = std::make_pair(robo_current.first + 1, robo_current.second);
          else
            current = std::make_pair(current.first + 1, current.second);
          break;
      }

      if(is_robo)
        deliveries[robo_current] += 1;
      else
        deliveries[current] += 1;

      is_robo = !is_robo;
    }
  }

  std::cout << deliveries.size() << '\n';

  return 0;
}