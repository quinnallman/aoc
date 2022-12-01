#include <iostream>

using namespace std;

int main(int argc, char **argv) {
  char direction = 0;
  int distance = 0;
  int x = 0, y = 0;
  int facing = 0;

  while(!cin.eof()) {
    cin >> direction >> distance;
    cin.ignore(2);

    if(direction == 'L') {
      facing++;
      facing %= 4;
    } else {
      facing--;
      if(facing < 0) facing += 4;
    }

    if(facing == 0) y -= distance;
    else if(facing == 1) x += distance;
    else if(facing == 2) y += distance;
    else if(facing == 3) x -= distance;
  }

  cout << abs(x + y) << endl;

  return 0;
}