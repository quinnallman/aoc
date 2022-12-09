#include <iostream>

using namespace std;

int main() {
  char buffer[4];
  cin >> buffer[0] >> buffer[1] >> buffer[2] >> buffer[3];
  if(buffer[0] != buffer[1] && buffer[0] != buffer[2] && buffer[0] != buffer[3] && buffer[1] != buffer[2] && buffer[1] != buffer[3] && buffer[2] != buffer[3]) {
    cout << "4" << endl;
    return 0;
  }

  int count = 4;
  while(!cin.eof()) {
    char in;
    cin >> in;
    count++;

    buffer[0] = buffer[1];
    buffer[1] = buffer[2];
    buffer[2] = buffer[3];
    buffer[3] = in;

    if(buffer[0] != buffer[1] && buffer[0] != buffer[2] && buffer[0] != buffer[3] && buffer[1] != buffer[2] && buffer[1] != buffer[3] && buffer[2] != buffer[3])
      break;
  }

  cout << count << endl;

  return 0;
}