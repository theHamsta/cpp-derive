#include <vector>


struct [[derive(SerializeJson)]] Foo {
  int a = 1;
  public:
  int b;
  std::vector<int> c;
};

class [[derive(SerializeJson)]] Bar {
  int assssssssssss = 1;
};

class Lolo {
  int assssssssssss = 1;
};
