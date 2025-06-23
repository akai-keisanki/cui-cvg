#include "cvg.hh"

// stdc++
#include <stdexcept>
#include <algorithm>

// generator class

CVG::CVG (void) {}

void CVG::add_character_group (const char& group)
{
  character_groups[group];
}

void CVG::remove_character_group (const char& group)
{
  character_groups.erase(group);
}

void CVG::clean_character_group (const char& group)
{
  auto group_it = character_groups.find(group);
  if (group_it == character_groups.end()) throw std::out_of_range("Unset character group");

  group_it->second.clear();
}

void CVG::character_group_add (const char& group, const wchar_t& character)
{
  auto group_it = character_groups.find(group);
  if (group_it == character_groups.end()) throw std::out_of_range("Unset character group");

  group_it->second.push_back(character);
}

void CVG::character_group_remove (const char& group, const wchar_t& character)
{
  auto group_it = character_groups.find(group);
  if (group_it == character_groups.end()) throw std::out_of_range("Unset character group");

  group_it->second.erase(std::find(group_it->second.begin(), group_it->second.end(), character));
}

void CVG::add_syllable_pattern (const char&, const std::string&)
{
  // TODO
}

void CVG::change_syllable_pattern (const char&, const std::string&)
{
  // TODO
}

void CVG::remove_syllable_pattern (const char&)
{
  // TODO
}

void CVG::add_word_pattern (const char&, const std::string&)
{
  // TODO
}

void CVG::change_word_pattern (const char&, const std::string&)
{
  // TODO
}

void CVG::remove_word_pattern (const char&)
{
  // TODO
}

wchar_t CVG::generate_char (const char&)
{
  // TODO
}

wchar_t CVG::generate_char (void)
{
  // TODO
}

std::wstring CVG::generate_syllable (const char&)
{
  // TODO
}

std::wstring CVG::generate_syllable (void)
{
  // TODO
}

std::wstring CVG::generate_word (const char&)
{
  // TODO
}

std::wstring CVG::generate_word (void)
{
  // TODO
}
