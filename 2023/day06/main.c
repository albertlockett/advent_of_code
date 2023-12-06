#include <math.h>
#include <stdio.h>

double get_wins(double* times, double* distances, int loops) {
  double wins_total = 1;

  for (int i = 0; i < loops; i++) {
    double terry = times[i] * times[i] - (distances[i] * 4.000000000000001);
    terry = sqrt(terry);

    double t1 = (times[i] + terry) / 2.0;
    double t2 = (times[i] - terry) / 2.0;

    double wins = floor(t1) - floor(t2);
    wins_total *= wins;
  }

  return wins_total;
}

double convert_to_p2(double* input, int size) {
  double result = 0;
  for (int i = 0; i < size; i++) {
    result *= pow(10, ceil(log10(input[i])));
    result += input[i];
  }
  return result;
}

int main() {
  // test input
  // double times[] = {7.0, 15.0, 30.0};
  // double distances[] = {9.0, 40.0, 200.0};

  // real input
  double times[] =     {  52.0,   94.0,   75.0,   94.0};
  double distances[] = { 426.0, 1374.0, 1279.0, 1216.0};

  int loops = sizeof(times) / sizeof(double);

  int p1_wins = get_wins(times, distances, loops);
  printf("p1_wins = %d\n", p1_wins);

  double p2_time = convert_to_p2(times, loops);
  double p2_distance = convert_to_p2(distances, loops);

  int p2_wins = get_wins(&p2_time, &p2_distance, 1);
  printf("p2_wins = %d\n", p2_wins);
}

