#include <math.h>
#include <stdio.h>

int main() {
  // test input
  // double times[] = {7.0, 15.0, 30.0};
  // double distances[] = {9.0, 40.0, 200.0};

  // real input
  double times[] =     {  52.0,   94.0,   75.0,   94.0};
  double distances[] = { 426.0, 1374.0, 1279.0, 1216.0};

  int wins_total = 1;

  int loops = sizeof(times) / sizeof(double);
  for (int i = 0; i < loops; i++) {
    double terry = times[i] * times[i] - (distances[i] * 4.000001);
    terry = sqrt(terry);

    double t1 = (times[i] + terry) / 2.0;
    double t2 = (times[i] - terry) / 2.0;

    double wins = floor(t1) - floor(t2);
    wins_total *= wins;
  }

  printf("wins_total = %d\n", wins_total);
}