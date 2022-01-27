
# C++ study

## Fundamental

- case sensitive

## keywords

alignas, alignof, and, and_eq, asm, auto, bitand, bitor, bool, break, case, catch, char, char16_t, char32_t, class, compl, const, constexpr, const_cast, continue, decltype, default, delete, do, double, dynamic_cast, else, enum, explicit, export, extern, false, float, for, friend, goto, if, inline, int, long, mutable, namespace, new, noexcept, not, not_eq, nullptr, operator, or, or_eq, private, protected, public, register, reinterpret_cast, return, short, signed, sizeof, static, static_assert, static_cast, struct, switch, template, this, thread_local, throw, true, try, typedef, typeid, typename, union, unsigned, using, virtual, void, volatile, wchar_t, while, xor, xor_eq

## variables and types

- **character types**: `char`, `char16_t`, `char32_t`, `wchar_t`
- **integer types (signed)**: `signed char`, `signed short` int, signed `int`, signed `long` int, signed `long long` int
- **integer types (unsigned)**: `unsigned char`, `unsigned short` int, `unsigned` int, `unsigned long` int, `unsigned long long` int
- **floating-point types**: `float`, `double`, `long double`
- **bolean types**: `bool`
- **void type**: `void` (no storage)
- **null pointer**: `decltype(nullptr)`

### Type size

definition for fixed-size type: `<cstdint>` or `<limits>`

## Declaration of variables

```c++
int a, b;
float c;
// Variable initialization with declaration
int x = 0;
int x (0); // type identifier (initial_value);
int x {0}; // type identifier {initial_value};
```

## Type deduction: auto and decltype

```c++
int foo = 0;
auto bar = foo;  // the same as: int bar = foo;
decltype(foo) bar;  // the same as: int bar;
```

## String

### Strings and null-terminated character sequences

In any case, null-terminated character sequences and strings are easily transformed from one another:

Null-terminated character sequences can be transformed into strings implicitly, and strings can be transformed into null-terminated character sequences by using either of string's member functions `c_str` or `data`:

```c++
char myntcs[] = "some text";
string mystring = myntcs;  // convert c-string to string
cout << mystring;          // printed as a library string
cout << mystring.c_str();  // printed as a c-string
```

## Constants

Constants are expressions with a fixed value.

```c++
75         // decimal
0113       // octal
0x4b       // hexadecimal
75         // int
75u        // unsigned int
75l        // long
75ul       // unsigned long
75lu       // unsigned long
3.14159    // 3.14159
6.02e23    // 6.02 x 10^23
1.6e-19    // 1.6 x 10^-19
3.0        // 3.0
3.14159L   // long double
6.02e23f   // float  
// character and string literals
'z'
'p'
"Hello world"
"How do you do?"
// escape code
"\n"	// newline
"\r"	// carriage return
"\t"	// tab
"\v"	// vertical tab
"\b"	// backspace
"\f"	// form feed (page feed)
"\a"	// alert (beep)
"\'"	// single quote (')
"\""	// double quote (")
"\?"	// question mark (?)
"\\"	// backslash (\)
"\x20" // hexadecimal digits character
"\x4A"
"this forms" "a single"     " string "
"of characters" // equivalent to "this formsa single string of characters"
// long string literals using backslash (\)
x = "string expressed in \
two lines" // x = "string expressed in two lines"

// different character type literals using prefix (not suffixes above)
// u	char16_t
// U	char32_t
// L	wchar_t
// u8 // UTF-8
R"(string with \backslash)" // raw string, literal format: R(VALUE)
R"&%$(string with \backslash)&%$" // = "string with \\backslash"

// other literals
bool foo = true;
bool bar = false;
int* p = nullptr;

// naming to a constant value
const double pi = 3.1415926;
const char tab = '\t';

// Preprocessor definitions (#define) - another mechanism for name constant
#define PI 3.14159
#define NEWLINE '\n'

```

## Casting

```c++
int i;
float f = 3.14;
i = (int) f;

i = int (f);
```

## sizeof

```c++
x = sizeof (char);
```
## Table of operation precedence

![Precedence of operators](https://i.stack.imgur.com/u3q2E.png)

## Basic input/output

- `cin`:	standard input stream
- `cout`:	standard output stream
- `cerr`:	standard error (output) stream
- `clog`:	standard logging (output) stream
- `stringstream`: string stream (make a string as a stream)

### Read a line with string stream

```c++
// stringstreams
#include <iostream>
#include <string>
#include <sstream>
using namespace std;

int main ()
{
  string mystr;
  float price=0;
  int quantity=0;

  cout << "Enter price: ";
  getline (cin,mystr);
  stringstream(mystr) >> price;
  cout << "Enter quantity: ";
  getline (cin,mystr);
  stringstream(mystr) >> quantity;
  cout << "Total price: " << price*quantity << endl;
  return 0;
}
```

## Range-based loop

The for-loop has another syntax, which is used exclusively with ranges:

`for ( declaration : range ) statement;`

```c++
// range-based for loop
#include <iostream>
#include <string>
using namespace std;

int main ()
{
  string str {"Hello!"};
  for (char c : str)
  {
    cout << "[" << c << "]";
  }
  cout << '\n';
}
// output: [H][e][l][l][o][!]
```

Range based loops usually also make use of type deduction for the type of the elements with `auto`. Typically, the range-based loop above can also be written as:

```c++
for (auto c : str)
  cout << "[" << c << "]";
```

Here, the type of c is automatically deduced as the type of the elements in str.


## The return value of main()

- `0` or	`EXIT_SUCCESS`:	The program was successful (same as above). This value is defined in header `<cstdlib>`.
- `EXIT_FAILURE`:	The program failed. This value is defined in header `<cstdlib>`.

## Argument pass by value and by reference

```c++
// passing parameters by reference
#include <iostream>
using namespace std;

void duplicate (int& a, int& b, int& c)
{
  a*=2;
  b*=2;
  c*=2;
}

int main ()
{
  int x=1, y=3, z=7;
  duplicate (x, y, z);
  cout << "x=" << x << ", y=" << y << ", z=" << z;
  return 0;
}
```

## Efficiency considerations and const references

```c++
string concatenate (const string& a, const string& b)
{
  return a+b;
}
```

By qualifying them as const, the function is forbidden to modify the values of neither a nor b, but can actually access their values as references (aliases of the arguments), without having to make actual copies of the strings.

Therefore, const references provide functionality similar to passing arguments by value, but with an increased efficiency for parameters of large types. That is why they are extremely popular in C++ for arguments of compound types. Note though, that for most fundamental types, there is no noticeable difference in efficiency, and in some cases, const references may even be less efficient!


## inline function
Preceding a function declaration with the inline specifier informs the compiler that inline expansion is preferred over the usual function call mechanism for a specific function.

For example, the concatenate function above may be declared inline as:

```c++
inline string concatenate (const string& a, const string& b)
{
  return a+b;
}
```

## function default value

```c++
int divide (int a, int b=2)
{
  int r;
  r=a/b;
  return (r);
}
int main ()
{
  cout << divide (12) << '\n';
  cout << divide (20,4) << '\n';
  return 0;
}
```

## C++ Generalization (Overloads and templates)

`overload`

The same function definition with different parameter types.

```c++
// overloading functions
#include <iostream>
using namespace std;

int operate (int a, int b)
{
  return (a*b);
}

double operate (double a, double b)
{
  return (a/b);
}

int main ()
{
  int x=5,y=2;
  double n=5.0,m=2.0;
  cout << operate (x,y) << '\n';
  cout << operate (n,m) << '\n';
  return 0;
}
```

`template <template-parameters> function-declaration`

The template parameters are a series of parameters separated by commas. These parameters can be generic template types by specifying either the class or typename keyword followed by an identifier. This identifier can then be used in the function declaration as if it was a regular type. For example, a generic sum function could be defined as:

```c++
// function template
#include <iostream>
using namespace std;

template <class T>
T sum (T a, T b)
{
  T result;
  result = a + b;
  return result;
}

int main () {
  int i=5, j=6, k;
  double f=2.0, g=0.5, h;
  k=sum<int>(i,j);
  h=sum<double>(f,g);

	// possible to skip angle brackets
	// k = sum (i,j);
	// h = sum (f,g);

  cout << k << '\n';
  cout << h << '\n';
  return 0;
}
```

Non-type template arguments (just support the constant expressions)

```c++
// template arguments
#include <iostream>
using namespace std;

template <class T, int N>
T fixed_multiply (T val)
{
  return val * N;
}

int main() {
  std::cout << fixed_multiply<int,2>(10) << '\n';
  std::cout << fixed_multiply<int,3>(10) << '\n';
}
```

## nampespace (Name visibility)


```c++
// namespaces
#include <iostream>
using namespace std;

namespace foo
{
  int value() { return 5; }
}

namespace bar
{
  const double pi = 3.1416;
  double value() { return 2*pi; }
}

int main () {
  cout << foo::value() << '\n';
  cout << bar::value() << '\n';
  cout << bar::pi << '\n';
  return 0;
}

// Namespaces can be split:
// Two segments of a code can be declared in the same namespace:

namespace foo { int a; }
namespace bar { int b; }
namespace foo { int c; }

```

## Compound data types

### Pointer

http://www.cplusplus.com/doc/tutorial/pointers/


- Address-of operator (`&`)
- Dereference operator (`*`)

### Pointers and const

Pointers can be used to access a variable by its address, and this access may include modifying the value pointed. But it is also possible to declare pointers that can access the pointed value to read it, but not to modify it. For this, it is enough with qualifying the type pointed to by the pointer as `const`. For example:

```c++
int x;
int y = 10;
const int * p = &y;
x = *p;          // ok: reading p
*p = x;          // error: modifying p, which is const-qualified
```


```c++
int x;
      int *       p1 = &x;  // non-const pointer to non-const int
const int *       p2 = &x;  // non-const pointer to const int
      int * const p3 = &x;  // const pointer to non-const int
const int * const p4 = &x;  // const pointer to const int

// To add a little bit more confusion to the syntax of const with pointers,
// the const qualifier can either precede or follow the pointed type,
// with the exact same meaning:
const int * p2a = &x;  //      non-const pointer to const int
int const * p2b = &x;  // also non-const pointer to const int
```

### NULL pointer

```c++
int * p = 0;
int * q = nullptr;
int * r = NULL;
```

### Pointers to functions

```c++
int (* func_name)(type,type,...);
```

## Dynamic memory

```c++
int * foo;
foo = new int [5];
// Access: foo[1] or *(foo+1)

// allocation
foo = new int [5];  // if allocation fails, an exception is thrown
foo = new (nothrow) int [5]; // return null without bad_alloc exception or terminating the program
if (foo == nullptr) {
  // error assigning memory. Take measures.
}

// deletion
delete pointer;
delete[] pointer;

```

## Data Structures

Skip...

## Type aliases

```c++
typedef char C;
typedef unsigned int WORD;
typedef char * pChar;
typedef char field [50];

C mychar, anotherchar, *ptc1;
WORD myword;
pChar ptc2;
field name;

// More recently, a second syntax to define type aliases was introduced in the C++ language:
// using new_type_name = existing_type ;

// For example, the same type aliases as above could be defined as:
using C = char;
using WORD = unsigned int;
using pChar = char *;
using field = char [50];

```

## Union type

Skip!

## Enumerated types (enum)

```c++
// ## Enumerated types mapping to convertible int.
enum colors_t {black=1, blue, green, cyan, red, purple, yellow, white};

colors_t mycolor;
mycolor = blue;
if (mycolor == green) mycolor = red;

// ## Enumerated types with enum class
enum class Colors {black, blue, green, cyan, red, purple, yellow, white};

Colors mycolor;
mycolor = Colors::blue;
if (mycolor == Colors::green) mycolor = Colors::red;

```

## class

- access_specifier: `private`, `public` or `protected`
 - `private` members of a class are accessible only from within other members of the same class (or from their "`friends`").
 - `protected` members are accessible from other members of the same class (or from their "`friends`"), but also from members of their derived classes.
 - Finally, `public` members are accessible from anywhere where the object is visible.

```c++
// classes example
#include <iostream>
using namespace std;

class Rectangle {
    int width, height;
  public:
    void set_values (int,int);
    int area() {return width*height;}
};

void Rectangle::set_values (int x, int y) {
  width = x;
  height = y;
}

int main () {
  Rectangle rect;
  rect.set_values (3,4);
  cout << "area: " << rect.area();
  return 0;
}
```

### Member initialization in constructors

```c++
class Rectangle {
    int width,height;
  public:
    Rectangle ();
    Rectangle(int,int);
    int area() {return width*height;}
};

Rectangle::Rectangle () {
  width = 5;
  height = 5;
}

Rectangle::Rectangle (int x, int y) { width=x; height=y; }
Rectangle::Rectangle (int x, int y) : width(x) { height=y; }
Rectangle::Rectangle (int x, int y) : width(x), height(y) { }
```

### Uniform initialization

- `class_name object_name = initialization_value;`
- `class_name object_name { value, value, value, ... }`
- `class_name object_name(value, value, value, ... )`

```c++
// classes and uniform initialization
#include <iostream>
using namespace std;

class Circle {
    double radius;
  public:
    Circle(double r) { radius = r; }
    double circum() {return 2*radius*3.14159265;}
};

int main () {
  Circle foo (10.0);   // functional form
  Circle bar = 20.0;   // assignment init.
  Circle baz {30.0};   // uniform init.
  Circle qux = {40.0}; // POD-like

  cout << "foo's circumference: " << foo.circum() << '\n';
  return 0;
}
```

### Overloading operators

#### Overloadable operators
```c++
+    -    *    /    =    <    >    +=   -=   *=   /=   <<   >>
<<=  >>=  ==   !=   <=   >=   ++   --   %    &    ^    !    |
~    &=   ^=   |=   &&   ||   %=   []   ()   ,    ->*  ->   new
delete    new[]     delete[]
```

## async and future

- Calls fn (with args as arguments) at some point, returning without waiting for the execution of fn to complete.
- http://www.cplusplus.com/reference/future/async/?kw=async


```c++
#include <cstdio>
#include <future> // std::async, std::future

using namespace std;

int fun() {
	for (int i = 1; i <= 10; i++) {
		printf("fun[%d]\n", i);
	}

	return 200;
}

int main() {
	//auto fut = async(fun);
	auto fut = async(launch::async, fun);

	// deferred until .get() or .wait()
	//auto fut = async(launch::deferred, fun);

	for (int i = 1; i <= 30; i++) {
		printf("main[%d]\t", i);
	}
	printf("\n");

	int result = fut.get();

	printf("result : %d\n", result);

	return 0;
}
```
