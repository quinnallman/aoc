#include <iostream>
#include <variant>
#include <deque>
#include <vector>

using namespace std;

struct item {
  bool is_int;
  variant<long, deque<item>> data;
};

typedef enum {
  number,
  comma,
  leftbracket,
  rightbracket,
} symbol;

struct token {
  symbol s;
  long v;
};

bool accept(symbol s, deque<token>& tokens) {
  auto token = tokens.front();
  if(s == token.s) {
    tokens.pop_front();
    return true;
  }

  return false;
}

bool expect(symbol s, deque<token>& tokens) {
  if(accept(s, tokens)) return true;
  cout << "Error: unexpected symbol " << tokens[0].s << ", expected " << s << endl;
  return false;
}

item anumber(deque<token>& tokens) {
  auto token = tokens.front();
  item ret = { .is_int = true, .data = token.v };
  tokens.pop_front();
  return ret;
}

item list(deque<token>& tokens) {
  deque<item> items;
  expect(leftbracket, tokens);
  do {
    auto token = tokens.front();
    if(token.s == leftbracket) {
      items.push_back(list(tokens));
    } else if(token.s == number) {
      items.push_back(anumber(tokens));
    }
  } while(accept(comma, tokens));
  expect(rightbracket, tokens);

  return { .is_int = false, .data = items };
}

deque<token> tokenize(string input) {
  deque<token> tokens;
  while(input.length()) {
    if(input[0] == '[') {
      tokens.push_back({ .s = leftbracket });
      input = input.substr(1);
    } else if(input[0] == ']') {
      tokens.push_back({ .s = rightbracket });
      input = input.substr(1);
    } else if(input[0] == ',') {
      tokens.push_back({ .s = comma });
      input = input.substr(1);
    } else {
      string::size_type sz;
      tokens.push_back({ .s = number, .v = stoi(input, &sz) });
      input = input.substr(sz);
    }
  }
  return tokens;
}

void parse(string input, item& item) {
  if(input[0] == '[') {
    item.is_int = false;
  } else {
    item.is_int = true;
    item.data = stoi(input);
  }
}

void print_item(item i) {
  if(i.is_int)
    cout << get<long>(i.data);
  else {
    cout << "[";
    for(auto i : get<deque<item>>(i.data)) {
      print_item(i);
      cout << ",";
    }
    cout << "]";
  }
}

long compare(item left, item right) {
  if(left.is_int && right.is_int) {
    return get<long>(left.data) - get<long>(right.data);
  } else if(left.is_int && !right.is_int) {
    deque<item> items;
    items.push_back({ .is_int = true, .data = get<long>(left.data) });
    item left_list { .is_int = false, .data = items };
    return compare(left_list, right);
  } else if(!left.is_int && right.is_int) {
    deque<item> items;
    items.push_back({ .is_int = true, .data = get<long>(right.data) });
    item right_list { .is_int = false, .data = items };
    return compare(left, right_list);
  } else {
    while(true) {
      auto ls = get<deque<item>>(left.data).size();
      auto rs = get<deque<item>>(right.data).size();

      if(ls < 1 && rs >= 1) return -1;
      else if(ls >= 1 && rs < 1) return 1;
      else if(ls < 1 && rs < 1) return 0;
      else {
        auto l = get<deque<item>>(left.data).front();
        get<deque<item>>(left.data).pop_front();
        auto r = get<deque<item>>(right.data).front();
        get<deque<item>>(right.data).pop_front();
        long ret = compare(l, r);
        if(ret != 0) {
          return ret;
        }
      }
    }
  }
}

struct {
  bool operator()(item a, item b) const { return compare(a, b) < 0; }
} customCompare;

int main() {
  string input;
  int i = 1;
  int sum = 0;
  vector<item> items;
  auto t2 = tokenize("[[2]]");
  auto l2 = list(t2);
  items.push_back(l2);

  auto t6 = tokenize("[[6]]");
  auto l6 = list(t6);
  items.push_back(l6);

  while(getline(cin, input)) {
    if(input.length() < 1) continue;

    auto tokens = tokenize(input);
    item item = list(tokens);
    items.push_back(item);
  }

  sort(items.begin(), items.end(), customCompare);

  int ix2, ix6;
  for(int ix = 0 ; ix < items.size() ; ix++) {
    auto i = items[ix];
    if(compare(i, l2) == 0) ix2 = ix + 1;
    if(compare(i, l6) == 0) ix6 = ix + 1;
  }

  cout << ix2*ix6 << endl;

  return 0;
}