#include <iostream>
#include <unordered_set>

int main() {
  size_t pc = 5;
  uint64_t regs[8] = {0};
  std::unordered_set<uint64_t> seen;
  uint64_t last_seen = 0;

  while (true) {
    if (regs[pc] == 0) { regs[3] = 123; }
    else if (regs[pc] == 1) { regs[3] = regs[3] & 456; }
    else if (regs[pc] == 2) { regs[3] = regs[3] == 72 ? 1 : 0; }
    else if (regs[pc] == 3) { regs[5] = regs[3] + regs[5]; }
    else if (regs[pc] == 4) { regs[5] = 0; }
    else if (regs[pc] == 5) { regs[3] = 0; }
    else if (regs[pc] == 6) { regs[1] = regs[3] | 65536; }
    else if (regs[pc] == 7) { regs[3] = 9450265; }
    else if (regs[pc] == 8) { regs[4] = regs[1] & 255; }
    else if (regs[pc] == 9) { regs[3] = regs[3] + regs[4]; }
    else if (regs[pc] == 10) { regs[3] = regs[3] & 16777215; }
    else if (regs[pc] == 11) { regs[3] = regs[3] * 65899; }
    else if (regs[pc] == 12) { regs[3] = regs[3] & 16777215; }
    else if (regs[pc] == 13) { regs[4] = 256 > regs[1] ? 1 : 0; }
    else if (regs[pc] == 14) { regs[5] = regs[4] + regs[5]; }
    else if (regs[pc] == 15) { regs[5] = regs[5] + 1; }
    else if (regs[pc] == 16) { regs[5] = 27; }
    else if (regs[pc] == 17) { regs[4] = 0; }
    else if (regs[pc] == 18) { regs[2] = regs[4] + 1; }
    else if (regs[pc] == 19) { regs[2] = regs[2] * 256; }
    else if (regs[pc] == 20) { regs[2] = regs[2] > regs[1] ? 1 : 0; }
    else if (regs[pc] == 21) { regs[5] = regs[2] + regs[5]; }
    else if (regs[pc] == 22) { regs[5] = regs[5] + 1; }
    else if (regs[pc] == 23) { regs[5] = 25; }
    else if (regs[pc] == 24) { regs[4] = regs[4] + 1; }
    else if (regs[pc] == 25) { regs[5] = 17; }
    else if (regs[pc] == 26) { regs[1] = regs[4]; }
    else if (regs[pc] == 27) { regs[5] = 7; }
    else if (regs[pc] == 28) {
      if (seen.find(regs[3]) != seen.end()) {
        std::cout << "Answer to part 2 is: " << last_seen << std::endl;
        break;
      }
      last_seen = regs[3];
      seen.insert(regs[3]);
      regs[4] = regs[3] == regs[0] ? 1 : 0;
    }
    else if (regs[pc] == 29) { regs[5] = regs[4] + regs[5]; }
    else if (regs[pc] == 30) { regs[5] = 5; }
    regs[pc] += 1;
  }
}

/*
#ip 5
seti 123 0 3
bani 3 456 3
eqri 3 72 3
addr 3 5 5
seti 0 0 5
seti 0 9 3
bori 3 65536 1
seti 9450265 6 3
bani 1 255 4
addr 3 4 3
bani 3 16777215 3
muli 3 65899 3
bani 3 16777215 3
gtir 256 1 4
addr 4 5 5
addi 5 1 5
seti 27 1 5
seti 0 9 4
addi 4 1 2
muli 2 256 2
gtrr 2 1 2
addr 2 5 5
addi 5 1 5
seti 25 7 5
addi 4 1 4
seti 17 5 5
setr 4 6 1
seti 7 8 5
eqrr 3 0 4
addr 4 5 5
seti 5 8 5
*/
