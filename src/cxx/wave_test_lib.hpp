#ifndef WAVE_TEST_LIB_HPP
#define WAVE_TEST_LIB_HPP

#ifndef __cplusplus
#error "This header file should only be included in C++ source file."
#endif

#include "wave_test_lib.h"
#include <string>

#if __cplusplus >= 201700
#define WAVE_ATTR_NO_RETURN [[noreturn]]
#else
#define WAVE_ATTR_NO_RETURN
#endif

#define DELETE_COPY_MOVE(cls)           \
  cls(const cls &) = delete;            \
  cls(cls &&) = delete;                 \
  cls& operator=(const cls &) = delete; \
  cls& operator=(cls &&) = delete;


namespace wave {


WAVE_ATTR_NO_RETURN
inline static void accept() {
  ::wave_accept();
}

WAVE_ATTR_NO_RETURN
inline static void accept(const char* msg) {
  ::wave_accept_msg(msg);
}

WAVE_ATTR_NO_RETURN
inline static void accept(const std::string& msg) {
  ::wave_accept_msg(msg.c_str());
}

WAVE_ATTR_NO_RETURN
inline static void reject(const char* msg) {
  ::wave_reject(msg);
}

WAVE_ATTR_NO_RETURN
inline static void reject(const std::string& msg) {
  ::wave_reject(msg.c_str());
}


namespace i {

class HandleWrapper {
protected:
  explicit HandleWrapper(::wave_handle handle)
    : _handle(handle)
  { }

  DELETE_COPY_MOVE(HandleWrapper)

  ::wave_handle handle() const {
    return _handle;
  }

private:
  ::wave_handle _handle;
};

template <typename D>
class HandleOwner : HandleWrapper {
protected:
  explicit HandleOwner(::wave_handle handle, D deleter)
    : HandleWrapper { handle }, 
      _deleter(deleter)
  { }

  virtual ~HandleOwner() {
    _deleter(handle());
  }
  
  DELETE_COPY_MOVE(HandleOwner)

private:
  D _deleter;
};

struct CheckerHandleDeleter {
  void operator()(::wave_handle handle) const {
    ::wave_checker_release(handle);
  }
};

struct InteractorHandleDeleter {
  void operator()(::wave_handle handle) const {
    ::wave_interactor_release(handle);
  }
};

} // namespace i


class JudgeReader : private i::HandleWrapper {
public:
  explicit JudgeReader(::wave_handle handle)
    : HandleWrapper { handle }
  { }

  DELETE_COPY_MOVE(JudgeReader)

  // TODO: Implement JudgeReader.
};

class Checker : private i::HandleOwner<i::CheckerHandleDeleter> {
public:
  explicit Checker()
    : HandleOwner { ::wave_checker_create(), i::CheckerHandleDeleter { } }
  { }

  DELETE_COPY_MOVE(Checker)
};


class Interactor : private i::HandleOwner<i::InteractorHandleDeleter> {
public:
  explicit Interactor()
    : HandleOwner { ::wave_interactor_create(), i::InteractorHandleDeleter { } }
  { }

  DELETE_COPY_MOVE(Interactor)
};


} // namespace wave


#endif
