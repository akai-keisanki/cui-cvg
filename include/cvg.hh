#ifndef CVG_HH
#define CVG_HH

// stdc++
#include <string>

// stdc++ STL
#include <vector>
#include <map>

// generator class
class CVG
{
protected:

  // character group
  std::map<char, std::vector<wchar_t>> syllable_groups;
  std::map<char, std::string> syllable_patterns;
  std::map<char, std::string> word_patterns;

public:

  CVG (void);

  void add_group (const char&);
  void remove_group (const char&);
  void clean_group (const char&);

  void group_add_character (const char&, const wchar_t&);
  void group_remove_character (const char&, const wchar_t&);

  void add_syllable_pattern (const char&, const std::string&);
  void change_syllable_pattern (const char&, const std::string&);
  void remove_syllable_pattern (const char&);

  void add_word_pattern (const char&, const std::string&);
  void change_word_pattern (const char&, const std::string&);
  void remove_word_pattern (const char&);

  wchar_t generate_char (const char&);
  wchar_t generate_char (void);
  std::wstring generate_syllable (const char&);
  std::wstring generate_syllable (void);
  std::wstring generate_word (const char&);
  std::wstring generate_word (void);

};

#endif
