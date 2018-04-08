/*
 * eecs230.h
 * based on std_lib_facilities.h from PPP/2e
 */

/*
    simple "Programming: Principles and Practice using C++ (second edition)" course header to
    be used for the first few weeks.
    It provides the most common standard headers (in the global namespace)
    and minimal exception/error support.

    Students: please don't try to understand the details of headers just yet.
    All will be explained. This header is primarily used so that you don't have
    to understand every concept all at once.

    By Chapter 10, you don't need this file and after Chapter 21, you'll understand it

    Revised April 25, 2010: simple_error() added

    Revised November 25 2013: remove support for pre-C++11 compilers, use C++11: <chrono>
    Revised November 28 2013: add a few container algorithms
    Revised June 8 2014: added #ifndef to workaround Microsoft C++11 weakness

    Revised Jan. 5, 2016: EECS230 version.

    Revised Jan. 27, 2016: added gtest include

    Revised Feb. 5, 2016: changed gtest to UnitTest++
*/

#pragma once

#include <algorithm>
#include <array>
#include <cmath>
#include <cstdlib>
#include <forward_list>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <list>
#include <random>
#include <regex>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

#include "UnitTest++/UnitTest++.h"

namespace eecs230 {

// This is very impolite.
using namespace std;

using Unicode = long;

template<class T>
string to_string(const T& t)
{
    ostringstream os;
    os << t;
    return os.str();
}

// exception for reporting the index involved in vector range errors
struct Range_error : out_of_range
{
    int index;
    Range_error(int i) : out_of_range("Range error: " + to_string(i)), index(i) { }
};

// trivially range-checked vector (no iterator checking)
template<class T>
struct Vector : public std::vector<T>
{
    using size_type = typename std::vector<T>::size_type;
    using std::vector<T>::vector;   // inheriting constructor

    T& operator[](unsigned int i) // rather than return at(i);
    {
        check(i);
        return std::vector<T>::operator[](i);
    }

    const T& operator[](unsigned int i) const
    {
        check(i);
        return std::vector<T>::operator[](i);
    }

private:
    void check(unsigned int i) const
    {
        if (this->size() <= i) throw Range_error(i);
    }
};

// disgusting macro hack to get a range checked vector:
#define vector Vector

// trivially range-checked string (no iterator checking):
struct String : std::string
{
    using size_type = std::string::size_type;

    char& operator[](unsigned int i) // rather than return at(i);
    {
        check(i);
        return std::string::operator[](i);
    }

    const char& operator[](unsigned int i) const
    {
        check(i);
        return std::string::operator[](i);
    }

private:
    void check(unsigned int i) const
    {
        if (size() <= i) throw Range_error(i);
    }
};

struct Exit : runtime_error
{
    Exit() : runtime_error("Exit") { }
};

// error() simply disguises throws:
inline void error(const string& s)
{
    throw runtime_error(s);
}

inline void error(const string& s, const string& s2)
{
    error(s + s2);
}

inline void error(const string& s, int i)
{
    ostringstream os;
    os << s << ": " << i;
    error(os.str());
}

template<class T>
char* as_bytes(T& i)  // needed for binary I/O
{
    void* addr = &i;    // get the address of the first byte
    // of memory used to store the object
    return static_cast<char*>(addr); // treat that memory as bytes
}

inline void keep_window_open()
{
//    cin.clear();
//    cout << "Please enter a character to exit\n";
//    char ch;
//    cin >> ch;
//    return;
}

inline void keep_window_open(string s)
{
//    if (s == "") return;
//    cin.clear();
//    cin.ignore(120, '\n');
//    for (; ;) {
//        cout << "Please enter " << s << " to exit\n";
//        string ss;
//        while (cin >> ss && ss != s)
//            cout << "Please enter " << s << " to exit\n";
//        return;
//    }
}

// error function to be used (only) until error() is introduced in Chapter 5:
inline void simple_error(string s)  // write ``error: s and exit program
{
    cerr << "error: " << s << '\n';
    keep_window_open();     // for some Windows environments
    exit(1);
}

// make std::min() and std::max() accessible on systems with antisocial macros:
#undef min
#undef max


// run-time checked narrowing cast (type conversion). See ???.
template<class R, class A>
R narrow_cast(const A& a)
{
    R r = R(a);
    if (A(r) != a) error(string("info loss"));
    return r;
}

// random number generators. See 24.7.



inline int randint(int min, int max)
{
    static default_random_engine ran;
    return uniform_int_distribution<>{min, max}(ran);
}

inline int randint(int max) { return randint(0, max); }

// container algorithms. See 21.9.

// Value_type<C> == C::value_type
template<typename C>
using Value_type = typename C::value_type;

// Iterator<C> == C::iterator
template<typename C>
using Iterator = typename C::iterator;

// Sort a container without exposing the client to iterators
template<typename C>
// requires Container<C>()
void sort(C& c)
{
    std::sort(c.begin(), c.end());
}

// Sort a container given a comparison, without exposing the client to
// iterators
template<typename C, typename Pred>
// requires Container<C>() && Binary_Predicate<Value_type<C>>()
void sort(C& c, Pred p)
{
    std::sort(c.begin(), c.end(), p);
}

// Find a value in a container without exposing the client to iterators
template<typename C, typename Val>
// requires Container<C>() && Equality_comparable<C,Val>()
Iterator<C> find(C& c, Val v)
{
    return std::find(c.begin(), c.end(), v);
}

// Find a value in a container according to a predicate, without exposing
// the client to iterators
template<typename C, typename Pred>
// requires Container<C>() && Predicate<Pred,Value_type<C>>()
Iterator<C> find_if(C& c, Pred p)
{
    return std::find_if(c.begin(), c.end(), p);
}

} // end namespace eecs230

// Never define anything in std--that's reserved for the system.
namespace std {

template<>
struct hash<eecs230::String>
{
    size_t operator()(const eecs230::String& s) const
    {
        return hash<std::string>()(s);
    }
};

}

// This is also very impolite.
using namespace eecs230;

