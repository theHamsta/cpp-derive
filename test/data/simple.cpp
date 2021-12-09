#include <vector>


struct [[derive(SerializeJson)]] Foo {
  int a = 1;
  public:
  int b;
  std::vector<int> c;
};
