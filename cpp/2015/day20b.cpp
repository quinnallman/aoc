#include <iostream>
#include <map>
#include <cmath>

using namespace std;

int main() {
  int input = 33100000;

  int house = 2;
  while(true) {
    int count = 11 + house * 11;

    for(int i = 2 ; i <= sqrt(house) ; i++) {
      if(house % i == 0) {
        if(house <= i * 50)
          count += i * 11;
        if(house / i != i) {
          if(house <= house / i * 50)
            count += house / i * 11;
        }
      }
    }

    if(count >= input) {
      cout << house << endl;
      break;
    }

    house++;
  }

  return 0;
}