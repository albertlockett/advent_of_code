#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

// queue of mask indexs impl ---------------------------
typedef struct queue {
  int     end_idx;
  int     start_idx;
  int     len;
  short*  vals;
} queue;

#define IS_Q_EMPTY(q) q->start_idx == q->end_idx

queue new_queue(int size) {
  queue q;
  q.end_idx = 0;
  q.start_idx = 0;
  q.vals = malloc(size * sizeof(short));
  q.len = size;
  return q;
}

void enqueue(queue* q, short val) {
  q->vals[q->end_idx++] = val;
  if (q->end_idx == q->len) {
    q->end_idx = 0;
  }
}

short dequeue(queue* q) {
  short val = q->vals[q->start_idx++];
  if (q->start_idx == q->len) {
    q->start_idx = 0;
  }
  return val;
}
// --------------------------------------------------------


// sequence impl ------------------------------------------
typedef struct sequence {
  int       len;
  short     mask[26];
  queue     q;
} sequence;

#define MASK_IDX(ch) (ch - 'a')

sequence new_sequence(int size) {
  sequence sq;
  sq.len = 0;
  sq.q = new_queue(size);

  // zero out the mask
  for (int i = 0; i < 26; i++) {
    sq.mask[i] = 0;
  }
  return sq;
}

int contains(sequence *sq, char ch) {
  return sq->mask[MASK_IDX(ch)] > 0;
}

void insert(sequence* sq, char ch) {
  enqueue(&sq->q, ch);
  if (!contains(sq, ch)) {
    sq->len++;
  }
  sq->mask[MASK_IDX(ch)]++;
}

void remove_oldest(sequence* sq) {
  char ch = dequeue(&sq->q);
  sq->mask[MASK_IDX(ch)]--;
  if (!contains(sq, ch)) {
    sq->len--;
  }
}
// --------------------------------------------------------

int main(int argc, char** argv) {
  int seq_len = 4;
  char ch;
  int seq_offset;
  sequence sq = new_sequence(seq_len);

  while(read(STDIN_FILENO, &ch, 1) > 0) {

    insert(&sq, ch);

    if (sq.len == seq_len) {
      break;
    }

    if (++seq_offset >= seq_len) {
      remove_oldest(&sq);
    }
  }

  printf("part 1 sequence at %d", seq_offset + 1);
}
