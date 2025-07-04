//
// Created by Liam Conrad on 7/4/25.
//

#ifndef VALIDATOR_H
#define VALIDATOR_H

#include <iostream>

namespace validator {
    int sumDigits(int n);

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
            bool isValid = true;

            for (int i = 0; i < std::strlen(data); i++) {
                if (data[i] == ' ') continue;

                isValid = std::isdigit(data[i]);
                if (!isValid) {
                    break;
                }
            }

            return isValid;
        }
        int fillDigits() {
            int count = 0;
            for (int i = 0; i < strlen(data); i++) {
                digits[i] = static_cast<int>(strtol((const char *) &data[i], nullptr, 10));
                count++;
            }
            for (int i = 0; i < strlen(data); i++) {
                std::cout << digits[i] << "\t" << std::endl;
            }
            return count;
        }

    // PUBLIC ------------------------------------------------------------------
    public:
        Validator() : data(), digits() {
            data[0] = '\0';
            for (int & digit : digits) {
                digit = 0;
            }
        };
        explicit Validator(const char* _data) : data(), digits() {
            strcpy(data, _data);
        };
        ~Validator() = default;

        [[nodiscard]] char* getData() const { return const_cast<char *>(data); }

        [[nodiscard]] Result isValid() const {
            if (!vetInput()) return ERR_BAD_INPUT;

            const int i = fillDigits();
            std::cout << "Number of digits: " << i << std::endl;

            return VALID;
        }
    }; // class Validator

} // namespace validator

#endif //VALIDATOR_H
