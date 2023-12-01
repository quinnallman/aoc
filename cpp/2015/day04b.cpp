#include <iostream>
#include "../libs/md5.h"

int main() {
  std::string secret;
  getline(std::cin, secret);

  for(int i = 1; ; i++) {
    std::string input = secret + std::to_string(i);
    MD5 result = MD5(input);
    std::string str = result.hexdigest();
    if(str[0] == '0' && str[1] == '0' && str[2] == '0' && str[3] == '0' && str[4] == '0' && str[5] == '0') {
      std::cout << i << '\n';
      break;
    }
  }

  return 0;
}