extern "C" {
  void help_debug_message(const char* file, const int line, const char* message);
}

#define __debug_message__(x) help_debug_message(__FILE__, __LINE__, x)
#define __log_message__(x) help_log_message(__FILE__, __LINE__, x)

typedef struct terminate_thread_s terminate_thread_t;

terminate_thread_t *
terminate_thread_create(void (*start)(void*), void* data);

void
terminate_thread_terminate(terminate_thread_t *thread);

// void
// terminate_thread_yield(terminate_thread_t *thread);

void
terminate_thread_drop(terminate_thread_t *thread);

