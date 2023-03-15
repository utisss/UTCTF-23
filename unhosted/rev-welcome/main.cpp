#include <Adafruit_LiquidCrystal.h>
int flag[] = {0, 117, 101, 119, 101, 102, 105, 95, 125, 123, 95, 116, 98, 109, 103, 108, 101, 111, 112, 104, 99, 111, 112, 98, 95, 97, 108, 101, 111};
int base = 11;
int len = 28;

Adafruit_LiquidCrystal out(0);

void setup() {
  out.begin(16, 2);
}

int f(long long x, unsigned int y, int p) {
    int res = 1;
    x %= p;
    if (x == 0) return 0;
 
    while (y > 0) {
        if (y & 1) res = (res * x) % p;
        y = y>>1;
        x = (x*x) % p;
    }
    return res;
}

void loop() {
  out.setCursor(0, 0);
  for (int i = 0; i < len; i++) {
    if (i == 16) out.setCursor(0,1);
  	out.print(char(flag[f(base, i, len)])); // Should be len + 1
  	delay(500);
  }
  out.clear();
}