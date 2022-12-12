#include <iostream>

using namespace std;

struct item {
  int cost;
  int damage;
  int armour;
};

struct character {
  int hp;
  int damage;
  int armour;
};

bool simulate(character player, character boss) {
  while(true) {
    boss.hp -= max(player.damage - boss.armour, 1);
    // cout << "The player deals " << player.damage << "-" << boss.armour << " = " << max(player.damage - boss.armour, 1) << " damage; the boss goes down to " << boss.hp << " hit points." << endl;
    if(boss.hp <= 0)
      return true;
    player.hp -= max(boss.damage - player.armour, 1);
    // cout << "The boss deals " << boss.damage << "-" << player.armour << " = " << max(boss.damage - player.armour, 1) << " damage; the player goes down to " << player.hp << " hit points." << endl;
    if(player.hp <= 0)
      return false;
  }
}

int main() {
  item items[] {
    { .cost =   8, .damage = 4, .armour = 0 }, // 0
    { .cost =  10, .damage = 5, .armour = 0 },
    { .cost =  25, .damage = 6, .armour = 0 },
    { .cost =  40, .damage = 7, .armour = 0 },
    { .cost =  74, .damage = 8, .armour = 0 }, // 4

    { .cost =  13, .damage = 0, .armour = 1 }, // 5
    { .cost =  31, .damage = 0, .armour = 2 },
    { .cost =  53, .damage = 0, .armour = 3 },
    { .cost =  75, .damage = 0, .armour = 4 },
    { .cost = 102, .damage = 0, .armour = 5 }, // 9

    { .cost =  25, .damage = 1, .armour = 0 }, // 10
    { .cost =  50, .damage = 2, .armour = 0 },
    { .cost = 100, .damage = 3, .armour = 0 },
    { .cost =  20, .damage = 0, .armour = 1 },
    { .cost =  40, .damage = 0, .armour = 2 },
    { .cost =  80, .damage = 0, .armour = 3 }, // 15
  };

  character player { .hp = 100, .damage = 0, .armour = 0 };
  character boss { .hp = 104, .damage = 8, .armour = 1 };

  int min = 1000;
  int cost = 0;
  for(int i = 0 ; i < 5 ; i++) {
    player.damage += items[i].damage;
    player.armour += items[i].armour;
    cost += items[i].cost;

    for(int a = 4 ; a < 10 ; a++)  {
      if(a > 4) {
        player.damage += items[a].damage;
        player.armour += items[a].armour;
        cost += items[a].cost;
      }

      for(int r1 = 9 ; r1 < 16 ; r1++) {
        if(r1 > 9) {
          player.damage += items[r1].damage;
          player.armour += items[r1].armour;
          cost += items[r1].cost;
        }

        for(int r2 = 9 ; r2 < 16 ; r2++) {
          if(r2 > 9 && r2 == r1) continue;
          if(r2 > 9) {
            player.damage += items[r2].damage;
            player.armour += items[r2].armour;
            cost += items[r2].cost;
          }

          if(simulate(player, boss) && cost < min) {
            min = cost;
          }

          if(r2 > 9) {
            player.damage -= items[r2].damage;
            player.armour -= items[r2].armour;
            cost -= items[r2].cost;
          }
        }

        if(r1 > 9) {
          player.damage -= items[r1].damage;
          player.armour -= items[r1].armour;
          cost -= items[r1].cost;
        }
      }

      if(a > 4) {
        player.damage -= items[a].damage;
        player.armour -= items[a].armour;
        cost -= items[a].cost;
      }
    }

    cost -= items[i].cost;
    player.armour -= items[i].armour;
    player.damage -= items[i].damage;
  }

  cout << min << endl;

  return 0;
}