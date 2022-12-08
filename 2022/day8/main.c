#include<unistd.h>
#include<stdlib.h>
#include<stdio.h>

// buffered reader to read in the grid
typedef struct buffered_grid_reader {
  short   grid_height;
  short   grid_width;
  int     total_size;
  int     buffer_len;
  int     num_buffers;
  int     num_alloc_buffers;
  short** buffers;
  int     curr_buffer_offset;
} buffered_grid_reader;

// initialize a new buffered reader to read the input from the grid
buffered_grid_reader new_buff_reader(int buffer_len) {
  buffered_grid_reader br;
  br.grid_height = 1;
  br.grid_width = -1;
  br.buffer_len = buffer_len;
  br.total_size = 0;
  
  br.num_buffers = 1;
  br.num_alloc_buffers = 1;
  // short* buffer = 
  br.buffers = malloc(br.num_alloc_buffers * sizeof(short*));
  br.buffers[0] = malloc(br.buffer_len * sizeof(short));
  br.curr_buffer_offset = 0;

  return br;
}

void init_new_buffer(buffered_grid_reader* br) {
  if (br->num_buffers == br->num_alloc_buffers) {
    // we've filled our list of pointers to buffers.. time to expand it
    short** new_buff_list = malloc(br->num_alloc_buffers * 2 * sizeof(short*));
    for (int i = 0; i < br->num_alloc_buffers; i++) {
      new_buff_list[i] = br->buffers[i];
    }
    free(br->buffers);
    br->buffers = new_buff_list;
    br->num_alloc_buffers *= 2;
  }
  br->buffers[br->num_buffers++] = malloc(br->buffer_len * sizeof(short));
  br->curr_buffer_offset = 0;
}

// insert a new tree into the grid reader
void insert_grid_val(buffered_grid_reader* br, char val) {
  br->total_size++;

  // check if current buffer has capacity or if we need to expand
  if (br->buffer_len == br->curr_buffer_offset) {
    init_new_buffer(br);
  }

  // if it's a line term, use it to figure out grid dimensions
  if (val == '\n') {
    br->grid_height++;
    if (br->grid_width == -1) {
      br->grid_width = br->total_size - 1;
    }
  }

  short tree_height = val - '0';
  br->buffers[br->num_buffers-1][br->curr_buffer_offset++] = tree_height;
}

int main() {
  buffered_grid_reader br = new_buff_reader(10); // TODO this is arbitrary

  char ch;
  while(read(STDIN_FILENO, &ch, 1) > 0) {
    insert_grid_val(&br, ch);
  }

  printf("grid height %d, width %d, total_size %d, num_buffers %d, num_aloc_buffers %d",
    br.grid_height,
    br.grid_width,
    br.total_size,
    br.num_buffers,
    br.num_alloc_buffers
  );

}
