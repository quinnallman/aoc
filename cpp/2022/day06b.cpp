#include <iostream>

using namespace std;

const int N = 14;

void print_buffer(char buffer[N]) {
  for(int i = 0 ; i < N ; i++)
    cout << buffer[i];
  cout << endl;
}

bool is_unique(char buffer[N]) {
  for(int i = 0 ; i < N ; i++)
    for(int j = i+1 ; j < N ; j++)
      if(buffer[i] == buffer[j]) {
        return false;
      }

  return true;
}

int main() {
  char buffer[N];
  for(int i = 0 ; i < N ; i++)
    cin >> buffer[i];

  int count = N;

  if(is_unique(buffer)) {
    cout << count << endl;
    return 0;
  }

  while(!cin.eof()) {
    char in;
    cin >> in;
    count++;

    for(int i = 0 ; i < N - 1; i++)
      buffer[i] = buffer[i+1];
    buffer[N-1] = in;

    if(is_unique(buffer))
      break;
  }

  cout << count << endl;

  return 0;
}