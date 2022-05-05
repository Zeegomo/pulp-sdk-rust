// Wrapper for some function declared as static in header files in the pulp-sdk
#include "stdint.h"
#include "pmsis.h"

void pi_cl_team_fork_wrap(int nb_cores, void (*entry)(void *), void *arg)
{
    pi_cl_team_fork(nb_cores, entry, arg);
}

void pi_cl_team_barrier_wrap()
{
    pi_cl_team_barrier();
}

void pi_cl_dma_cmd_wrap(uint32_t ext, uint32_t loc, uint32_t size, pi_cl_dma_dir_e dir, pi_cl_dma_cmd_t *cmd) {
    pi_cl_dma_cmd(ext, loc,size,dir, cmd);
}

void abort_all(){
  exit(1);
}

void pi_cl_dma_wait_wrap(void* copy){
  pi_cl_dma_wait(copy);
}

int main() {
  return 0;
}