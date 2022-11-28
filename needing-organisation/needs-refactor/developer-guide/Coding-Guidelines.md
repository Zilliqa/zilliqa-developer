# General Development Practices

Here are a few operational guidelines to keep everyone synchronized:

1. **We will use C++14** and avoid the need for other third-party libraries. A rundown of new features in C++14 can be found [[here|https://blog.smartbear.com/c-plus-plus/the-biggest-changes-in-c11-and-why-you-should-care/]]. The most important usable features are smart pointers and the OS-agnostic threading library.
2. **Fix all compiler warnings.** `-Wall` will be turned on during compilation. Developers must address all warnings before pushing new code.
3. **Avoid duplicate code**. We will maintain a set of reusable utility libraries at 'src/libUtils' such as for logging, tokenizing, and time-locked functions.
4. **Respect the interface**. We should maintain a highly-cohesive/loosely-coupled design. If we need to expose some inner workings, a redesign should be done.
5. **Handle errors systematically**:
   - _**If it's a violation based on a contract between developers, use assert**_. For example, C++ functions like strcpy() do not bother checking for NULL inputs anymore. We will activate `-DDEBUG` and use assertions during dev phase to quickly detect bugs.
   - _**If it's an operational error without a recovery means, throw an exception**_. Examples include resource allocation and I/O errors.
   - _**If it's an operational error that should be handled during normal execution, use a return value**_.

# Secure Coding

A comprehensive list of C++ secure coding practices can be found [[here|https://www.securecoding.cert.org/confluence/pages/viewpage.action?pageId=637]]. We need to emphasize the following:

1. **Use smart pointers and avoid raw resource management**.

   - Refer to #1 in the Common Mistakes section for some common memory leak examples.
   - Refer to #1 and #2 in the Sample Code section for examples on how to use smart pointers.

2. **Check for bounds**. This is another cause of memory leaks. Note that most C++ constructs do not check for bounds. Refer to #2 in the Common Mistakes section for an example using vector.

3. **Avoid deadlocks by ordering all locks in the program**. A function/process/thread who wants to acquire lock B has to first acquire all locks above B in the hierarchy.

4. **Do not acquire/release locks manually**. Use the C++14 constructs `mutex` and `lock_guard` as demonstrated [[here|http://www.bogotobogo.com/cplusplus/C11/7_C11_Thread_Sharing_Memory.php]].

# Functional Coding Tips

1. Declare const all unmodified non-native types.
2. Prefer the compiler over the preprocessor.

```
#define MIN_VALUE 1000               // cannot infer from this what our range is
const unsigned int min_value = 1000; // here we know its range is unsigned int

```

# Performance Coding Tips

1. Use pass-by-ref for all non-native types:

```
void foo(A a);         // this will incur a copy construction penalty
void foo(const A & a); // no penalty (use const if a should not be modified)
```

2. Use class initialization section for non-native types whenever possible:

```
class myClass
{
    A a1;
    A a2;
public:
    myClass(const A & src) : a1(src) // no penalty (copy constructor will be used)
    {
        a2 = src; // this will incur default construction penalty
    }
};
```

3. A quick way to clear a buffer:

```
char buf1[10];
memset(buf1, 0, 10); // clears buf1

char buf2[10] = {0}; // same effect
```

4. C++11 has introduced `move` and rvalue semantics which significantly improves performance by avoiding temporary object creation (for example, during swapping). See Sample Code section for examples.

# Formatting and Coding Style

We need to maintain a consistent coding style to make reviews easier, and to give the impression that the product is professionally developed by a cohesive unit.

1. **Use 4 spaces for indents**, not tab (editor-specific rendering) or fewer spaces (harder to read).
2. **Put opening/closing braces in their own lines**.
3. **Always have opening/closing braces** even for single-line branch blocks.
4. **Don't indent preprocessor directives**.

```
#define minval 0       // #4

class myClass
{                      // #2
public:
    myClass(int x);    // #1
    ~myClass();
private:
    int x;
};

myClass::myClass(int src) : x(src)
{
    if (src < minval)
    {
        src = minval;  // #3
    }
}
```

5. **Use these templates** when creating a new C++ file:

something.cpp template

```
#include <standard header 1>
#include <standard header 2>
#include "own header 1"
#include "own header 2"

using namespace xxx;

```

something.h template

```
#ifndef __SOMETHING_H__
#define __SOMETHING_H__

#include <standard header 1>
#include <standard header 2>
#include "own header 1"
#include "own header 2"

#endif // __SOMETHING_H__
```

6. **Naming convention**: use `m_varName` for member variables and `funcName` for member functions.

# Common Mistakes

1. **Memory leaks** due to manual resource management:

```
void bar(char ** x)
{
    *x = new char[10];
}

// Case 1: Missing deallocation
void foo(char * dst)
{
    char * a = NULL;
    bar(&a);
    strcpy(dst, a, 10);
} // a is leaked here

// Case 2: Wrong deallocation
void foo()
{
    char * a = NULL;
    bar(&a);
    delete a; // should be delete [] a
}

// Case 3: Double free
void foo()
{
    char * a = NULL;
    bar(&a);
    delete [] a;
    // 100 lines of code follow
    delete [] a;
}
```

2. **No bounds checking**. Here is an example using vector:

```
#include <iostream>
#include <vector>

using namespace std;

int main()
{
    vector<char> x(5, 'A');

    // No bounds check
    try
    {
        cout << "Fifth element = " << x[5] << "." << endl;
    }
    catch(...)
    {
        cout << "Failed 1" << endl;
    }

    // Has bounds check
    try
    {
        cout << "Fifth element = " << x.at(5) << "." << endl;
    }
    catch(...)
    {
        cout << "Failed 2" << endl;
    }

    return 0;
}

Output:
Fifth element = .
Failed 2
```

3. **Sending zero-sized message thru socket** --> ``write()` will stall the program.
4. **Wrong use of break and continue** --> `break` will terminate a loop, `continue` will proceed to the next loop iteration
5. **Wrong use of sizeof and strlen** --> using `sizeof()` on any pointer (char*, int*, etc.) will return either 4 or 8 bytes depending on 32/64-bit OS
6. **Not using opening/closing braces** --> can lead to wrong code logic when code is updated:

```
Code first version:
if (something)
    do_something();

Code subsequently updated:
if (something)
    do_something();
    do_another_thing(); // will be run regardless of "if" condition
```

# Sample Code

1. `unique_ptr` - smart pointer where there is only 1 owner for the resource. Once out of scope, resource is deallocated.

```
#include <memory>
#include <iostream>

using namespace std;

unique_ptr<int[]> make_me_an_array(const unsigned int size)
{
    return make_unique<int[]>(size);
}

void write_to_the_array(unique_ptr<int[]> & dst, const unsigned int size)
{
    for (int i=0; i<size; i++)
    {
        dst[i] = i;
    }
}

void print_the_array(const unique_ptr<int[]> & dst, const unsigned int size)
{
    if (dst != nullptr)
    {
        for (int i=0; i<size; i++)
        {
            cout << dst[i] << " ";
        }
        cout << endl;
    }
    else
    {
        cout << "It's null!" << endl;
    }
}

void transfer_ownership(unique_ptr<int[]> dst)
{
    // It's mine now!
    // Will be freed when I go out of scope
}

int main()
{
    unique_ptr<int[]> x = make_me_an_array(5);

    write_to_the_array(x, 5);
    print_the_array(x, 5);
    transfer_ownership(move(x));
    print_the_array(x, 5);

    return 0;
}

Output:
0 1 2 3 4
It's null!
```

2. `shared_ptr` - Multiple owners for 1 resource. Once the last owner goes out of scope, the resource is deallocated.

3. Thread creation

4. `condition_variable` - A way to sync update of shared data between threads. See [[here|http://en.cppreference.com/w/cpp/thread/condition_variable]] for an example.

5. Timers

6. Lambdas - anonymous functions

7. `move` - avoids temp object creation by internally referencing the pointer to the old data to point to the new data

8. Initialization - these are now possible in C++11:

```
vector<int> x = { 1, 2, 3 };

class A
{
    int m_var = 5;
};
```
