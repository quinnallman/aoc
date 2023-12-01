#include <iostream>
#include <string>
#include <openssl/md5.h>

using namespace std;

void myhash(const string& plain, char result[33]) {
  unsigned char digest[MD5_DIGEST_LENGTH];

  MD5((unsigned char *) plain.c_str(), plain.size(), digest);

  for(int i = 0 ; i < 16 ; i++) {
    sprintf(&result[i*2], "%02x", (unsigned int) digest[i]);
  }
}

int main() {
  std::string base = "uqwqemis";
  std::string potential;
  int count = 0;
  char result[33];
  char password[8] = {'F', 'F', 'F', 'F', 'F', 'F', 'F', 'F'};

  for(long long i = 0 ; count < 8 ; i++) {
    potential = base + std::to_string(i);

    myhash(potential, result);

    if(result[0] == '0' && result[1] == '0' && result[2] == '0' && result[3] == '0' && result[4] == '0') {
      int position = result[5] - '0';
      if(position > 7 || password[position] != 'F') continue;
      password[position] = result[6];
      count++;
    }
  }

  for(int i = 0 ; i < 8 ; i++)
    cout << password[i];
  cout << endl;
}