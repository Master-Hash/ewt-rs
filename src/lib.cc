#if 0 && defined(_MSC_VER)
import std;
import winrt.Windows.Data.Text;
import winrt.Windows.Fundation.Collections;
import winrt.Windows.Foundation;
#else
#include <winrt/Windows.Data.Text.h>
#include <winrt/Windows.Foundation.Collections.h>
#include <winrt/Windows.Foundation.h>
#endif

namespace emacs {
#if __has_include(<emacs-module.h>)
#include <emacs-module.h>
#else
#include "../emacs-module.h"
#endif
} // namespace emacs

using emacs::emacs_env, emacs::emacs_value, emacs::emacs_runtime;
using namespace winrt;
using namespace Windows::Data::Text;
using namespace Windows::Foundation;
using namespace Windows::Foundation::Collections;

namespace {
static SelectableWordsSegmenter g_segmenter =
    SelectableWordsSegmenter(L"zh-CN");

auto copy_string(emacs_env *__restrict env,
                 emacs_value __restrict arg) noexcept {
  ptrdiff_t len = 0;
  if (!env->copy_string_contents(env, arg, nullptr, &len)) {
    // throw std::runtime_error("copy_string_contents: failed to get length");
    std::abort();
  }
  std::string buffer(len, '\0');
  if (!env->copy_string_contents(env, arg, buffer.data(), &len)) {
    // throw std::runtime_error("copy_string_contents: failed to copy content");
    std::abort();
  }
  // if (!buffer.empty() && buffer.back() == '\0') {
  //     buffer.pop_back();
  // }
  return buffer;
}

auto Femt__do_split_helper(emacs_env *__restrict env, ptrdiff_t nargs,
                           emacs_value *__restrict args,
                           void *__restrict data) noexcept {
  (void)nargs;
  (void)data;
  auto text = copy_string(env, args[0]);
  auto text_hstring = winrt::to_hstring(text);

  auto tokens = g_segmenter.GetTokens(text_hstring);
  std::vector<emacs_value> conses;
  auto count = tokens.Size();
  for (uint32_t i = 0; i < count; ++i) {
    auto token = tokens.GetAt(i);
    auto segment = token.SourceTextSegment();
    int64_t start = segment.StartPosition;
    int64_t end = segment.StartPosition + segment.Length;

    auto l = env->make_integer(env, start);
    auto r = env->make_integer(env, end);
    emacs_value cons_args[2] = {l, r};
    auto cons = env->funcall(env, env->intern(env, "cons"), 2, cons_args);
    conses.push_back(cons);
  }

  return env->funcall(env, env->intern(env, "vector"),
                      static_cast<ptrdiff_t>(conses.size()), conses.data());
}

auto Femt__word_at_point_or_forward(emacs_env *__restrict env, ptrdiff_t nargs,
                                    emacs_value *__restrict args,
                                    void *__restrict data) noexcept {
  (void)nargs;
  (void)data;
  auto text = copy_string(env, args[0]);
  auto pos = env->extract_integer(env, args[1]);
  auto text_hstring = winrt::to_hstring(text);

  auto token = g_segmenter.GetTokenAt(text_hstring, static_cast<uint32_t>(pos));
  auto segment = token.SourceTextSegment();
  int64_t start = segment.StartPosition;
  int64_t end = segment.StartPosition + segment.Length;

  auto l = env->make_integer(env, start);
  auto r = env->make_integer(env, end);
  emacs_value cons_args[2] = {l, r};
  return env->funcall(env, env->intern(env, "cons"), 2, cons_args);
}
} // namespace

extern "C" int emacs_module_init(emacs_runtime *ert) {
  winrt::init_apartment(winrt::apartment_type::single_threaded);

  auto *env = ert->get_environment(ert);

  auto intern = [env](const char *name) { return env->intern(env, name); };
  auto funcall = [env](emacs_value fn, int nargs, emacs_value args[]) {
    return env->funcall(env, fn, nargs, args);
  };

  auto Qfset = intern("fset");
  auto Qsplit_helper = intern("emt--do-split-helper");
  auto Qword_at_point = intern("emt--word-at-point-or-forward-helper");

  auto func_split = env->make_function(
      env, 1, 1, Femt__do_split_helper,
      "This function takes a string and return an array of bounds. "
      "A bound is a cons with the starting position and the ending position of "
      "a word.",
      nullptr);
  emacs_value fset_args1[2] = {Qsplit_helper, func_split};
  funcall(Qfset, 2, fset_args1);

  auto func_word =
      env->make_function(env, 2, 2, Femt__word_at_point_or_forward,
                         "This functions takes a string and a position, and "
                         "returns the bound of the word at the position. "
                         "If the position is at bound of two words, it returns "
                         "the word at the right side of that position. "
                         "This function does not tokenize the whole string, so "
                         "it is faster in some cases.",
                         nullptr);
  emacs_value fset_args2[2] = {Qword_at_point, func_word};
  funcall(Qfset, 2, fset_args2);

  return 0;
}

extern "C" int plugin_is_GPL_compatible = 1;
