#include <iostream>
#include <string>
#include <openssl/md5.h>

using namespace std;

int main() {
  std::string base = "uqwqemis";
  std::string potential;
  unsigned char digest[MD5_DIGEST_LENGTH];
  int count = 0;

  for(long long i = 0 ; count < 8 ; i++) {
    potential = base + std::to_string(i);
    MD5((unsigned char *)potential.c_str(), potential.size(), digest);

    char result[33];

    for(int j = 0 ; j < 16 ; j++) {
      sprintf(&result[j*2], "%02x", (unsigned int)digest[j]);
    }

    if(result[0] == '0' && result[1] == '0' && result[2] == '0' && result[3] == '0' && result[4] == '0') {
      std::cout << result[5] << flush;
      count++;
    }
  }

  cout << endl;
}