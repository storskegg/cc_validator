//
// Created by Liam Conrad on 7/4/25.
//

#include "validator.h"

namespace validator {
    int sumDigits(int n) {
        int sum = 0;
        while (n > 0) {
            sum += n % 10;
            n /= 10;
        }
        return sum;
    }
} // validator