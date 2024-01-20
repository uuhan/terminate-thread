
typedef struct terminate_thread_s terminate_thread_t;

terminate_thread_t *
terminate_thread_create(void (*start)(void*), void* data);

void
terminate_thread_terminate(terminate_thread_t *thread);

void
terminate_thread_yield(terminate_thread_t *thread);

void
terminate_thread_drop(terminate_thread_t *thread);

