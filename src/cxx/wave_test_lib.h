#ifndef WAVE_TEST_LIB_H
#define WAVE_TEST_LIB_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus


typedef void* wave_handle;

typedef int32_t wave_bool;

#define WAVE_TRUE   (1)
#define WAVE_FALSE  (0)

#define WAVE_DEFAULT_TOLERANCE  (1e-8)

typedef int32_t wave_partial_ordering;
typedef int32_t wave_ordering;

#define WAVE_LESS           (-1)
#define WAVE_EQUAL          (0)
#define WAVE_GREATER        (1)
#define WAVE_NOT_COMPARABLE (2147483647)


void 
wave_accept();

void 
wave_accept_msg(
  const char* msg
);

void
wave_reject(
  const char* msg
);

wave_handle
wave_checker_create();

void
wave_checker_release(
  wave_handle checker
);

wave_handle
wave_checker_get_input_handle(
  wave_handle checker
);

wave_handle
wave_checker_get_std_answer_handle(
  wave_handle checker
);

wave_handle
wave_checker_get_user_answer_handle(
  wave_handle checker
);

wave_handle
wave_interactor_create();

void
wave_interactor_release(
  wave_handle interactor
);

wave_handle
wave_interactor_get_input_handle(
  wave_handle interactor
);

wave_handle
wave_interactor_get_answer_handle(
  wave_handle interactor
);

wave_handle
wave_interactor_get_read_end_handle(
  wave_handle interactor
);

wave_handle
wave_interactor_get_write_end_handle(
  wave_handle interactor
);

size_t
wave_read_token(
  wave_handle handle,
  void* buffer,
  size_t buffer_size
);

size_t
wave_read_line(
  wave_handle handle,
  void* buffer,
  size_t buffer_size
);

int8_t
wave_expect_i8(
  wave_handle handle
);

uint8_t
wave_expect_u8(
  wave_handle handle
);

int16_t
wave_expect_i16(
  wave_handle handle
);

uint16_t
wave_expect_u16(
  wave_handle handle
);

int32_t
wave_expect_i32(
  wave_handle handle
);

uint32_t
wave_expect_u32(
  wave_handle handle
);

int64_t
wave_expect_i64(
  wave_handle handle
);

uint64_t
wave_expect_u64(
  wave_handle handle
);

void
wave_expect_token(
  wave_handle handle,
  const char* expected,
  wave_bool ignore_case
);

void 
wave_expect_signed(
  wave_handle handle,
  int64_t expected
);

void
wave_expect_unsigned(
  wave_handle handle,
  uint64_t expected
);

void
wave_expect_fp(
  wave_handle handle,
  double expected,
  double tolerance
);

void
wave_expect_eof(
  wave_handle handle
);

wave_partial_ordering
wave_cmp_fp(
  double actual,
  double expected,
  double tolerance
);

wave_ordering
wave_cmp_str(
  const char* actual,
  const char* expected
);

wave_bool
wave_cmp_str_eq(
  const char* actual,
  const char* expected
);


#ifdef __cplusplus
}
#endif  // __cplusplus

#endif // WAVE_TEST_LIB_H
