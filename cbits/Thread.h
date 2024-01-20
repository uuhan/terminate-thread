#pragma once

#include "config.h"

#ifndef HAVE_PTHREAD
#  error "Libdjvu requires thread support"
# endif


// ----------------------------------------
// INCLUDES

#include <sys/types.h>
#include <sys/time.h>
#include <unistd.h>
#include <pthread.h>


class Thread {
public:
  /** Constructs a new thread object.  Memory is allocated for the
      thread, but the thread is not started. 
      Argument #stacksize# is used by the #COTHREADS# model only for
      specifying the amount of memory needed for the processor stack. A
      negative value will be replaced by a suitable default value of 128Kb.
      A minimum value of 32Kb is silently enforced. */
  Thread(int stacksize = -1);

  /** Destructor.  Destroying the thread object while the thread is running is
      perfectly ok since it only destroys the thread identifier.  Execution
      will continue without interference. */
  ~Thread();

  /** Starts the thread. The new thread executes function #entry# with
      argument #arg#.  The thread terminates when the function returns.  A
      thread cannot be restarted after its termination. You must create a new
      #Thread# object. */
  int create(void (*entry)(void*), void *arg);

  /** Terminates a thread with extreme prejudice. The thread is removed from
      the scheduling list.  Execution terminates regardless of the execution
      status of the thread function. Automatic variables may or may not be
      destroyed. This function must be considered as a last resort since
      memory may be lost. */
  void terminate();

  /** Causes the current thread to relinquish the processor.  The scheduler
      selects a thread ready to run and transfers control to that thread.  The
      actual effect of #yield# heavily depends on the selected implementation.
      Function #yield# usually returns zero when the execution of the current
      thread is resumed.  It may return a positive number when it can
      determine that the current thread will remain the only runnable thread
      for some time.  You may then call function \Ref{get_select} to
      obtain more information. */
  static int yield();

  /** Returns a value which uniquely identifies the current thread. */
  static void *current();
private:
  pthread_t hthr;
  static void *start(void *arg);
public:
  // Should be considered as private
  void (*xentry)(void*);
  void  *xarg;
private:
  // Disable default members
  Thread(const Thread&);
  Thread& operator=(const Thread&);
};

