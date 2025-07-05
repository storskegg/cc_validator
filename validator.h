//
// Created by Liam Conrad on 7/4/25.
//

#ifndef VALIDATOR_H
#define VALIDATOR_H

#include <iostream>

namespace validator {
    #define SK_VAL_DATA_LEN 64L // Maximum length of input data
    constexpr size_t MAX_DATA = SK_VAL_DATA_LEN;

    int sumDigits(int n);
    int toggleMultiplier(int n);

    enum Result {
        VALID = 0, // Input passes Luhn algorithm
        INVALID = 1, // Input fails Luhn algorithm
        ERR_BAD_INPUT = 2, // Input contains invalid characters
        // Add more results as needed
    };

    class Validator {
    // PRIVATE -----------------------------------------------------------------
    private:
        char data[SK_VAL_DATA_LEN];
        int digits[SK_VAL_DATA_LEN];

    /**
     * vetInput() vets the input data's integrity. At present, only digits and
     * spaces (ASCII 0x20) are allowed.
     *
     * @return boolean indicating whether the input data is clean/valid.
     */
    [[nodiscard]] bool vetInput() const {
            for (int i = 0; i < std::strlen(data); i++) {
                if (data[i] == ' ') continue;

                if (!std::isdigit(data[i])) {
                    return false;
                }
            }

            return true;
        } // vetInput()

    /**
     * fillDigits() reads input from the data[] array, and populates the
     * digits[] array using only the valid digits. (This accounts for the use
     * of spaces in, say, a credit card number.)
     *
     * @return Count of the number of values updated in the digits[] array.
     */
    [[nodiscard]] int fillDigits() {
            int count = 0;

            for (int i = 0; i < strlen(data); i++) {
                if (!std::isdigit(data[i])) continue;
                digits[count++] = data[i] - '0';
            }

            return count;
        } // fillDigits()

    /**
     * LuhnSum performs the sumative portion of the Luhn algorithm based on the
     * values of the digits[] array. Starting at the second to last valid number,
     * the first number is multiplied by 2, the second by 1, and so on; and
     * the products are added together. This sum is then returned.
     *
     * @return Luhn sum of the digits (see Luhn algorithm)
     */
    [[nodiscard]] int LuhnSum() const {
            int sum = 0;

            const int last = static_cast<int>(strlen(data)) - 1;
            int currentMultiplier = toggleMultiplier(0);

            for (int i = last-1; i >= 0; i--) {
                sum += sumDigits(digits[i] * currentMultiplier);
                currentMultiplier = toggleMultiplier(currentMultiplier);
            }

            return sum;
        } // LuhnSum()

    // PUBLIC ------------------------------------------------------------------
    public:
        // Constructors and Destructor
        explicit Validator(const char* _data) : data(), digits() {
            std::strncpy(data, _data, sizeof(data)-1);
            data[sizeof(data)-1] = '\0';
            std::fill(std::begin(digits), std::end(digits), 0);
        }
        Validator() : Validator("") {}
        ~Validator() = default;

        // Class Methods
        [[nodiscard]] char* getData() const { return const_cast<char *>(data); }
        [[nodiscard]] int getModulus() const { return digits[strlen(data) - 1]; }

        void printDigits() const {
            std::cout << "Digits: " << data << std::endl;
        } // printDigits()

    /**
     * isValid() first vets the input data's integrity, then fills the digits[]
     * array, and finally performs the Luhn algorithm to determine if the
     * given number is valid or not. Both the given and calculated Luhn modulus
     * are displayed for verification purposes.
     *
     * @return Result indicating whether the input data is valid (VALID) or not
     */
    [[nodiscard]] Result isValid() {
            if (!vetInput()) return ERR_BAD_INPUT;

            const int i = fillDigits();
            std::cout << "Number of digits: " << i << std::endl;

            const int sum = LuhnSum();
            const int lm = (10 - (sum % 10)) % 10;

            std::cout << "Calculated Luhn modulus: " << lm << std::endl;
            std::cout << "Existing Luhn modulus:   " << getModulus() << std::endl;

            if (lm != digits[strlen(data) - 1]) return INVALID;

            return VALID;
        } // isValid()
    }; // class Validator

} // namespace validator

#endif //VALIDATOR_H
