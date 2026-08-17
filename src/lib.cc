#ifdef USE_WINRT
#if 0
import std;
import winrt.Windows.Data.Text;
import winrt.Windows.Fundation.Collections;
#else
#include <winrt/Windows.Data.Text.h>
#include <winrt/Windows.Foundation.Collections.h>
#endif
#elifdef USE_ICU
#define U_CHARSET_IS_UTF8 1
#include <vector>
#include <unicode/brkiter.h>
#include <unicode/unistr.h>
#endif

namespace emacs {
#if __has_include(<emacs-module.h>)
#include <emacs-module.h>
#else
#include "../emacs-module.h"
#endif
} // namespace emacs

using emacs::emacs_env, emacs::emacs_value, emacs::emacs_runtime;

#ifdef USE_WINRT
using namespace winrt;
using namespace Windows::Data::Text;
using namespace Windows::Foundation;
using namespace Windows::Foundation::Collections;
#elifdef USE_ICU
#endif

namespace {

#ifdef USE_WINRT
auto g_segmenter =
    SelectableWordsSegmenter(L"zh-CN");
#elifdef USE_ICU
auto status = U_ZERO_ERROR;
auto bi = []() {
  const auto t = icu::BreakIterator::createWordInstance(
      icu::Locale::getSimplifiedChinese(), status);
  if (U_FAILURE(status)) {
    std::abort();
  }
  return t;
}();
#endif

#ifdef USE_WINRT
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
  return buffer;
}
#elifdef USE_ICU
auto copy_string(emacs_env *__restrict env,
                 const emacs_value __restrict arg) noexcept -> std::u8string {
  ptrdiff_t len = 0;
  if (!env->copy_string_contents(env, arg, nullptr, &len)) {
    std::abort();
  }
  std::u8string buffer(len, u8'\0');
  if (!env->copy_string_contents(env, arg,
                                 reinterpret_cast<char *>(&buffer[0]), &len)) {
    std::abort();
  }
  // if (buffer.size() > static_cast<size_t>(len)) {
  //   buffer.resize(len);
  // }
  return buffer;
}

struct ByteToCharConverter {
  const std::u8string &text;
  int64_t last_byte = 0;
  int64_t last_char = 0;

  explicit ByteToCharConverter(const std::u8string &t) : text(t) {
  }

  auto convert(int64_t byte_offset) noexcept -> int64_t {
    const auto size = static_cast<int64_t>(text.size());
    while (last_byte < byte_offset && last_byte < size) {
      if ((static_cast<unsigned char>(text[last_byte]) & 0xC0) != 0x80) {
        ++last_char;
      }
      ++last_byte;
    }
    return last_char;
  }
};

auto char_to_byte_index(const std::u8string &text,
                        const int64_t char_index) noexcept -> int64_t {
  int64_t i = 0;
  int64_t count = 0;
  const auto size = static_cast<int64_t>(text.size());
  while (i < size && count < char_index) {
    ++i;
    while (i < size && (static_cast<unsigned char>(text[i]) & 0xC0) == 0x80) {
      ++i;
    }
    ++count;
  }
  return i;
}
#endif

auto Femt__do_split_helper(emacs_env *__restrict env, const ptrdiff_t nargs,
                           emacs_value *__restrict args,
                           void *__restrict data) noexcept {
  (void)nargs;
  (void)data;
  const auto text = copy_string(env, args[0]);
  std::vector<emacs_value> conses;

  auto pushlr = [&conses, env](auto start, auto end) {
    const auto l = env->make_integer(env, start);
    const auto r = env->make_integer(env, end);
    emacs_value cons_args[2] = {l, r};
    const auto cons = env->funcall(env, env->intern(env, "cons"), 2, cons_args);
    conses.push_back(cons);
  };
#ifdef USE_WINRT
  const auto text_hstring = winrt::to_hstring(text);
  const auto tokens = g_segmenter.GetTokens(text_hstring);

  const auto count = tokens.Size();
  for (uint32_t i = 0; i < count; ++i) {
    auto token = tokens.GetAt(i);
    const auto [StartPosition, Length] = token.SourceTextSegment();
    const int64_t start = StartPosition;
    const int64_t end = StartPosition + Length;

    pushlr(start, end);
  }
#elifdef USE_ICU
  const auto text_icu = utext_openUTF8(nullptr,
                                       reinterpret_cast<const char *>(text.
                                         c_str()), text.length(),
                                       &status);
  bi->setText(text_icu, status);
  ByteToCharConverter conv(text);
  auto start = bi->first();
  for (auto end = bi->next(); end != icu::BreakIterator::DONE;
       start = end, end = bi->next()) {
    pushlr(conv.convert(start), conv.convert(end));
  }
#endif

  return env->funcall(env, env->intern(env, "vector"),
                      static_cast<ptrdiff_t>(conses.size()), conses.data());
}

auto Femt__word_at_point_or_forward(emacs_env *__restrict env,
                                    const ptrdiff_t nargs,
                                    emacs_value *__restrict args,
                                    void *__restrict data) noexcept {
  (void)nargs;
  (void)data;
  const auto text = copy_string(env, args[0]);
  const auto pos = env->extract_integer(env, args[1]);
#ifdef USE_WINRT
  const auto text_hstring = winrt::to_hstring(text);

  const auto token = g_segmenter.GetTokenAt(text_hstring,
                                            static_cast<uint32_t>(pos));
  const auto [StartPosition, Length] = token.SourceTextSegment();
  const int64_t start = StartPosition;
  const int64_t end = StartPosition + Length;
#elifdef USE_ICU
  const auto text_icu = utext_openUTF8(nullptr,
                                       reinterpret_cast<const char *>(text.
                                         c_str()), text.length(),
                                       &status);
  bi->setText(text_icu, status);
  const auto pos_byte = char_to_byte_index(text, pos);
  int64_t start, end;
  if (bi->isBoundary(pos_byte)) {
    start = pos_byte;
    const auto e = bi->next();
    end = e == icu::BreakIterator::DONE
            ? text.length()
            : e;
  } else {
    end = bi->current();
    const auto s = bi->previous();
    start = s == icu::BreakIterator::DONE ? 0 : s;
  }
  ByteToCharConverter conv(text);
  start = conv.convert(start);
  end = conv.convert(end);
#endif
  const auto l = env->make_integer(env, start);
  const auto r = env->make_integer(env, end);
  emacs_value cons_args[2] = {l, r};
  return env->funcall(env, env->intern(env, "cons"), 2, cons_args);
}
} // namespace

extern "C" int emacs_module_init(emacs_runtime *ert) {
#ifdef USE_WINRT
  winrt::init_apartment(winrt::apartment_type::single_threaded);
#endif

  auto *env = ert->get_environment(ert);

  auto intern = [env](const char *name) { return env->intern(env, name); };
  auto funcall = [env](const emacs_value fn, const int nargs,
                       emacs_value args[]) {
    return env->funcall(env, fn, nargs, args);
  };

  const auto Qfset = intern("fset");
  const auto Qsplit_helper = intern("emt--do-split-helper");
  const auto Qword_at_point = intern("emt--word-at-point-or-forward-helper");

  const auto func_split = env->make_function(
      env, 1, 1, Femt__do_split_helper,
      "This function takes a string and return an array of bounds. "
      "A bound is a cons with the starting position and the ending position of "
      "a word.",
      nullptr);
  emacs_value fset_args1[2] = {Qsplit_helper, func_split};
  funcall(Qfset, 2, fset_args1);

  const auto func_word =
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
