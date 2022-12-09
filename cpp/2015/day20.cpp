#include <iostream>
#include <map>
#include <cmath>

using namespace std;

int main() {
  int input = 33100000;

  int house = 2;
  while(true) {
    int count = 10 + house * 10;

    for(int i = 2 ; i <= sqrt(house) ; i++) {
      if(house % i == 0) {
        count += i * 10;
        if(house / i != i) {
          count += house / i * 10;
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