#include <iostream>
#include <vector>

using namespace std;

const int N = 8;
const int NUM_ROUNDS = 10000;

struct monkey {
  int index;
  vector<long long> worries;
  char op;
  bool operand_is_old;
  long long operand;
  int test_divisor;
  int true_throw;
  int false_throw;
  int inspect_count;
};

long long calc_worry(monkey m, int worry_index) {
  long long old = m.worries[worry_index];
  long long operand = m.operand_is_old ? old : m.operand;
  if(m.op == '+')
    return old + operand;
  return old * operand;
}

int main() {
  monkey monkies[N];
  int test_divisor_modulus = 1;

  string line;
  while(getline(cin, line)) {
    int index = stoi(line.substr(7, 1));
    monkies[index].index = index;

    getline(cin, line);
    string values = line.substr(18);
    while(values.length() > 0) {
      int worry = 0;
      if(values.find(',') != string::npos) {
        worry = stoi(values.substr(0, values.find(',')));
        values = values.substr(values.find(',') + 2);
      } else {
        worry = stoi(values);
        values = "";
      }
      monkies[index].worries.push_back(worry);
    }

    getline(cin, line);
    monkies[index].op = line[23];
    if(line.substr(25) == "old")
      monkies[index].operand_is_old = true;
    else {
      monkies[index].operand_is_old = false;
      monkies[index].operand = stoi(line.substr(25));
    }

    getline(cin, line);
    monkies[index].test_divisor = stoi(line.substr(21));
    test_divisor_modulus *= monkies[index].test_divisor;

    getline(cin, line);
    monkies[index].true_throw = line[line.length() - 1] - '0';

    getline(cin, line);
    monkies[index].false_throw = line[line.length() - 1] - '0';

    getline(cin, line);
    monkies[index].inspect_count = 0;
  }

  for(int round = 0 ; round < NUM_ROUNDS ; round++) {
    //cout << round << endl;
    for(int m = 0 ; m < N ; m++) {
      if(monkies[m].worries.size() <= 0) continue;

      // cout << "Monkey " << m << endl;
      for(int i = 0 ; i < monkies[m].worries.size() ; i++) {
        // cout << "  Monkey inspects an item with a worry level of " << monkies[m].worries[i] << "." << endl;
        long long new_worry = calc_worry(monkies[m], i) % test_divisor_modulus;
        int new_monkey;
        if(new_worry % monkies[m].test_divisor == 0) {
          // cout << "    Current worry level is divisible by " << monkies[m].test_divisor << "." << endl;
          new_monkey = monkies[m].true_throw;
        } else {
          // cout << "    Current worry level is not divisible by " << monkies[m].test_divisor << "." << endl;
          new_monkey = monkies[m].false_throw;
        }
        // cout << "    Item with worry level " << new_worry << " is thrown to monkey " << new_monkey << "." << endl;
        monkies[new_monkey].worries.push_back(new_worry);
        monkies[m].inspect_count++;
      }

      monkies[m].worries.clear();
    }

    // if(round == 0 || round == 19 || (round+1)%1000 == 0) {
    //   cout << "== After round " << round + 1 << " ==" << endl;
    //   for(int i = 0 ; i < N ; i++) {
    //     cout << "Monkey " << i << " inspected items " << monkies[i].inspect_count << " times." << endl;
    //   }
    //   cout << endl;
    // }
  }

  long long most[2] = {0, 0};
  for(int i = 0 ; i < N ; i++) {
    // cout << "Monkey " << i << " inspected items " << monkies[i].inspect_count << " times." << endl;
    if(monkies[i].inspect_count > most[0]) {
      most[1] = most[0];
      most[0] = monkies[i].inspect_count;
    } else if(monkies[i].inspect_count > most[1]) {
      most[1] = monkies[i].inspect_count;
    }
  }

  cout << most[0] << " * " << most[1] << " = " << most[0] * most[1] << endl;

  return 0;
}