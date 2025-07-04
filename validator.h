//
// Created by Liam Conrad on 7/4/25.
//

#ifndef VALIDATOR_H
#define VALIDATOR_H

#include <iostream>

namespace validator {
    int sumDigits(int n);
    int toggleMultiplier(const int n);

    enum Result {
        VALID = 0,
        INVALID = 1,
        ERR_BAD_INPUT = 2,
        // Add more results as needed
    };

    class Validator {
    // PRIVATE -----------------------------------------------------------------
    private:
        char data[32];
        int digits[32];

        [[nodiscard]] bool vetInput() const {
            for (int i = 0; i < std::strlen(data); i++) {
                if (data[i] == ' ') continue;

                if (!std::isdigit(data[i])) {
                    return false;
                }
            }

            return true;
        }
        [[nodiscard]] int fillDigits() {
            int count = 0;

            for (int i = 0; i < strlen(data); i++) {
                digits[count++] = data[i] - '0';
            }

            for (int i = 0; i < strlen(data); i++) {
                std::cout << digits[i] << "\t" << std::endl;
            }

            return count;
        }
        int LuhnSum() {
            int sum = 0;

            int last = static_cast<int>(strlen(data)) - 1;
            int currentMultiplier = toggleMultiplier(0);

            for (int i = last-1; i >= 0; i--) {
                sum += sumDigits(digits[i] * currentMultiplier);
                std::cout << "== Mult...." << currentMultiplier << std::endl;
                std::cout << "== Digit..." << digits[i] << std::endl;
                std::cout << "== Sum....." << sum << std::endl;
                currentMultiplier = toggleMultiplier(currentMultiplier);
            }

            return sum;
        }

    // PUBLIC ------------------------------------------------------------------
    public:
        Validator() : data(), digits() {
            data[0] = '\0';
            std::fill(std::begin(digits), std::end(digits), 0);
        };
        explicit Validator(const char* _data) : data(), digits() {
            std::strncpy(data, _data, sizeof(data)-1);
            data[sizeof(data)-1] = '\0';
            std::fill(std::begin(digits), std::end(digits), 0);
        };
        ~Validator() = default;

        [[nodiscard]] char* getData() const { return const_cast<char *>(data); }
        [[nodiscard]] const int* getDigits() const { return digits; }

        [[nodiscard]] Result isValid() {
            if (!vetInput()) return ERR_BAD_INPUT;

            const int i = fillDigits();
            std::cout << "Number of digits: " << i << std::endl;

            const int sum = LuhnSum();

            int lm = (10 - (sum % 10)) % 10;

            if (lm != digits[strlen(data) - 1]) return INVALID;

            return VALID;
        }
    }; // class Validator

} // namespace validator

#endif //VALIDATOR_H
