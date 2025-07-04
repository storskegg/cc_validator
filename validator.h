//
// Created by Liam Conrad on 7/4/25.
//

#ifndef VALIDATOR_H
#define VALIDATOR_H

#include <iostream>

namespace validator {
    #define SK_VAL_DATA_LEN 64L

    constexpr size_t MAX_DATA = SK_VAL_DATA_LEN;

    int sumDigits(int n);
    int toggleMultiplier(int n);

    enum Result {
        VALID = 0,
        INVALID = 1,
        ERR_BAD_INPUT = 2,
        // Add more results as needed
    };

    class Validator {
    // PRIVATE -----------------------------------------------------------------
    private:
        char data[SK_VAL_DATA_LEN];
        int digits[SK_VAL_DATA_LEN];

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

            return count;
        }
        [[nodiscard]] int LuhnSum() const {
            int sum = 0;

            const int last = static_cast<int>(strlen(data)) - 1;
            int currentMultiplier = toggleMultiplier(0);

            for (int i = last-1; i >= 0; i--) {
                sum += sumDigits(digits[i] * currentMultiplier);
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
        [[nodiscard]] int getModulus() const { return digits[strlen(data) - 1]; }
        void printDigits() const {
            std::cout << "Digits: " << data << std::endl;
        }

        [[nodiscard]] Result isValid() {
            if (!vetInput()) return ERR_BAD_INPUT;

            const int i = fillDigits();
            std::cout << "Number of digits: " << i << std::endl;

            const int sum = LuhnSum();
            const int lm = (10 - (sum % 10)) % 10;

            std::cout << "Calculated Luhn modulus: " << lm << std::endl;
            std::cout << "Existing Luhn modulus: " << getModulus() << std::endl;

            if (lm != digits[strlen(data) - 1]) return INVALID;

            return VALID;
        }
    }; // class Validator

} // namespace validator

#endif //VALIDATOR_H
