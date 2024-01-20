
typedef struct stoppable_thread_s stoppable_thread_t;

stoppable_thread_t *
stoppable_thread_create(void (*start)(void*), void* data);

void
stoppable_thread_terminate(stoppable_thread_t *thread);

void
stoppable_thread_yield(stoppable_thread_t *thread);

void
stoppable_thread_drop(stoppable_thread_t *thread);

