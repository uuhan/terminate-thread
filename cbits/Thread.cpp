#include "wrapper.h"
#include "Thread.h"

#if defined(__linux__) && !defined(__ANDROID__)

#include <cxxabi.h>
#define FIX_UNWIND catch (abi::__forced_unwind&) { throw; }

#else // __linux__

#define FIX_UNWIND 

#endif // __linux__

static pthread_t pthread_null; // portable zero initialization!

void *
Thread::start(void *arg)
{
  Thread *gt = (Thread*)arg;

# ifdef PTHREAD_CANCEL_ENABLE
  pthread_setcancelstate(PTHREAD_CANCEL_ENABLE, 0);
# endif

# ifdef PTHREAD_CANCEL_ASYNCHRONOUS
  pthread_setcanceltype(PTHREAD_CANCEL_ASYNCHRONOUS, 0);
# endif

  try 
  {
    (gt->xentry)(gt->xarg);
  }
  FIX_UNWIND
  catch(...)
  {
  }
  return 0;
}


Thread::Thread(int) : 
  hthr(pthread_null), xentry(0), xarg(0)
{
}

Thread::~Thread()
{
  hthr = pthread_null;
}

int  
Thread::create(void (*entry)(void*), void *arg)
{
  if (xentry || xarg)
    return -1;
  xentry = entry;
  xarg = arg;
  pthread_attr_t attr;
  pthread_attr_init(&attr);
  pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
  int ret = pthread_create(&hthr, &attr, start, (void*)this);
  pthread_attr_destroy(&attr);
  return ret;
}

void 
Thread::terminate()
{
#ifndef __ANDROID__
  if (xentry || xarg)
    pthread_cancel(hthr);
#endif
}

int
Thread::yield()
{
  // should use sched_yield() when available.
  static struct timeval timeout = { 0, 0 };
  ::select(0, 0,0,0, &timeout);
  return 0;
}

void*
Thread::current()
{
  pthread_t self = pthread_self();
  return (void*) self;
}




// ----------------------------------------
//  EXPORT TO RUST
// ----------------------------------------

struct terminate_thread_s
{
  Thread thr;

  ~terminate_thread_s()
  {
    if (thr.xarg || thr.xentry)
      thr.terminate();
  }
};

terminate_thread_t *
terminate_thread_create(void (*start)(void*), void* data)
{
  auto thread = new terminate_thread_s;
  thread->thr.create(start, data);
  return thread;
}

void
terminate_thread_terminate(terminate_thread_t *thread)
{
  thread->thr.terminate();
}

void
terminate_thread_yield(terminate_thread_t *thread)
{
  thread->thr.yield();
}

void
terminate_thread_drop(terminate_thread_t *thread)
{
    delete thread;
}

