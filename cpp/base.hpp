#ifndef BASE_HPP
#define BASE_HPP
#include <iostream>

class Base {
public:
    Base();
    virtual ~Base() = default;
    virtual void hello() const;
};

#endif