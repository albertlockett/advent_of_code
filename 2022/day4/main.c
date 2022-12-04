#include <stdio.h>
#include <unistd.h>

#define INCR(target, digit_char) \
    target = target * 10 + digit_char - '0'

#define DO_CONTINIUE(continue_iter) continue_iter > 0
#define STOP_ITER(contine_iter) *continue_iter = 0
#define INIT_ITER_CONTINUE(contine_iter) contine_iter = 1;

struct region_limits {
    u_int8_t lower;
    u_int8_t upper;
};

struct elf_region_pair {
    struct region_limits elf_one;
    struct region_limits elf_two;
};

struct elf_region_pair parse_next_pair(int* continue_iter) {
    struct elf_region_pair pair;
    pair.elf_one.lower = 0;
    pair.elf_one.upper = 0;
    pair.elf_two.lower = 0;
    pair.elf_two.upper = 0;

    u_int8_t* struct_start = &pair;
    int offset = 0;

    char ch;
    while(read(0, &ch, 1) > 0){
        if ('-' == ch || ',' == ch || '-' == ch) {
            offset++;
            continue;
        }

        if (ch >='0' && ch <= '9') {
            INCR(struct_start[offset], ch);
            continue;
        }

        if ('\n' == ch) {
            return pair;
        }
    }
    STOP_ITER(continue_iter);
}

int is_region_contained(struct elf_region_pair pair) {
    if (
        pair.elf_one.lower <= pair.elf_two.lower && 
        pair.elf_one.upper >= pair.elf_two.upper
    ) {
        return 1;
    }
    if (
        pair.elf_two.lower <= pair.elf_one.lower && 
        pair.elf_two.upper >= pair.elf_one.upper
    ) {
        return 1;
    }
    return 0;
}

int main(int argc, char** argv) {
    int regions_contained = 0;
    struct elf_region_pair pair;
    int continue_iter;

    INIT_ITER_CONTINUE(continue_iter);
    while (DO_CONTINIUE(continue_iter)) {
        pair = parse_next_pair(&continue_iter);
        regions_contained += is_region_contained(pair);
    }
    
    printf("part 1 = there are %d contained regions\n", regions_contained);
    return 0;
}